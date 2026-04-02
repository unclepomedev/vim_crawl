use crate::ast::action::Action;
use crate::ast::command::ParsedCommand;
use crate::ast::motion::Motion;
use crate::ast::operator::Operator;
use crate::ast::target::Target;
use crate::error::ParseError;
use crate::parser::VimParser;
use crate::parser::mapping::{parse_motion, parse_operator, parse_pending_action, try_parse_count};
use crate::parser::result::ParseResult;
use crate::state::Mode;

fn combined_context(ctx: &crate::state::CommandContext) -> crate::state::CommandContext {
    let combined = match (ctx.operator_count, ctx.count) {
        (Some(a), Some(b)) => Some(a.saturating_mul(b)),
        (Some(a), None) => Some(a),
        (None, Some(b)) => Some(b),
        (None, None) => None,
    };
    crate::state::CommandContext {
        count: combined,
        operator_count: None,
        register: ctx.register,
        pending_action: None,
    }
}

pub fn handle(parser: &mut VimParser, op: Operator, c: char) -> ParseResult {
    if try_parse_count(c, &mut parser.state.context) {
        return ParseResult::Incomplete;
    }

    if let Some(pending) = parse_pending_action(c) {
        parser.state.context.pending_action = Some(pending);
        return ParseResult::Incomplete;
    }

    if let Some(second_op) = parse_operator(c) {
        return handle_operator(parser, op, second_op);
    }

    if let Some(motion) = parse_motion(c) {
        return handle_motion(parser, op, motion);
    }

    parser.state.mode = Mode::Normal;
    ParseResult::Invalid(ParseError::InvalidMotion)
}

pub fn handle_operator(
    parser: &mut VimParser,
    first_op: Operator,
    second_op: Operator,
) -> ParseResult {
    if first_op == second_op {
        parser.state.mode = match first_op {
            Operator::Change => Mode::Insert,
            _ => Mode::Normal,
        };

        let action = Action::Operate(first_op, Target::Line);
        let command = ParsedCommand {
            context: combined_context(&parser.state.context),
            action,
        };
        ParseResult::Success(command)
    } else {
        parser.state.mode = Mode::Normal;
        ParseResult::Invalid(ParseError::UnknownCommand)
    }
}

pub fn handle_motion(parser: &mut VimParser, op: Operator, motion: Motion) -> ParseResult {
    parser.state.mode = match op {
        Operator::Change => Mode::Insert,
        _ => Mode::Normal,
    };

    let action = Action::Operate(op, Target::Motion(motion));
    let command = ParsedCommand {
        context: combined_context(&parser.state.context),
        action,
    };
    ParseResult::Success(command)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::motion::Motion;
    use crate::state::{CommandContext, PendingAction};

    #[test]
    fn test_operator_pending_line_wise_dd() {
        let mut parser = VimParser::new();
        parser.state.mode = Mode::OperatorPending(Operator::Delete);

        let result = handle(&mut parser, Operator::Delete, 'd');

        assert_eq!(
            result,
            ParseResult::Success(ParsedCommand {
                context: CommandContext::default(),
                action: Action::Operate(Operator::Delete, Target::Line),
            })
        );
        assert_eq!(parser.state.mode, Mode::Normal);
    }

    #[test]
    fn test_operator_pending_line_wise_yy() {
        let mut parser = VimParser::new();
        parser.state.mode = Mode::OperatorPending(Operator::Yank);

        let result = handle(&mut parser, Operator::Yank, 'y');

        assert_eq!(
            result,
            ParseResult::Success(ParsedCommand {
                context: CommandContext::default(),
                action: Action::Operate(Operator::Yank, Target::Line),
            })
        );
        assert_eq!(parser.state.mode, Mode::Normal);
    }

    #[test]
    fn test_operator_pending_line_wise_cc() {
        let mut parser = VimParser::new();
        parser.state.mode = Mode::OperatorPending(Operator::Change);

        let result = handle(&mut parser, Operator::Change, 'c');

        assert_eq!(
            result,
            ParseResult::Success(ParsedCommand {
                context: CommandContext::default(),
                action: Action::Operate(Operator::Change, Target::Line),
            })
        );
        assert_eq!(parser.state.mode, Mode::Insert);
    }

    #[test]
    fn test_operator_pending_invalid_dy() {
        let mut parser = VimParser::new();
        parser.state.mode = Mode::OperatorPending(Operator::Delete);

        let result = handle(&mut parser, Operator::Delete, 'y');

        assert_eq!(result, ParseResult::Invalid(ParseError::UnknownCommand));
        assert_eq!(parser.state.mode, Mode::Normal);
    }

    #[test]
    fn test_operator_pending_valid_motion_dw() {
        let mut parser = VimParser::new();
        parser.state.mode = Mode::OperatorPending(Operator::Delete);

        let result = handle(&mut parser, Operator::Delete, 'w');

        assert_eq!(
            result,
            ParseResult::Success(ParsedCommand {
                context: CommandContext::default(),
                action: Action::Operate(Operator::Delete, Target::Motion(Motion::WordForward)),
            })
        );
        assert_eq!(parser.state.mode, Mode::Normal);
    }

    #[test]
    fn test_operator_pending_valid_motion_cw() {
        let mut parser = VimParser::new();
        parser.state.mode = Mode::OperatorPending(Operator::Change);

        let result = handle(&mut parser, Operator::Change, 'w');

        assert_eq!(
            result,
            ParseResult::Success(ParsedCommand {
                context: CommandContext::default(),
                action: Action::Operate(Operator::Change, Target::Motion(Motion::WordForward)),
            })
        );
        assert_eq!(parser.state.mode, Mode::Insert);
    }

    #[test]
    fn test_operator_pending_invalid_motion() {
        let mut parser = VimParser::new();
        parser.state.mode = Mode::OperatorPending(Operator::Delete);

        let result = handle(&mut parser, Operator::Delete, 'z');

        assert_eq!(result, ParseResult::Invalid(ParseError::InvalidMotion));
        assert_eq!(parser.state.mode, Mode::Normal);
    }

    #[test]
    fn test_operator_pending_with_count() {
        let mut parser = VimParser::new();
        parser.state.mode = Mode::OperatorPending(Operator::Delete);

        let result1 = handle(&mut parser, Operator::Delete, '3');
        assert_eq!(result1, ParseResult::Incomplete);

        let result2 = handle(&mut parser, Operator::Delete, 'w');
        assert_eq!(
            result2,
            ParseResult::Success(ParsedCommand {
                context: CommandContext {
                    count: Some(3),
                    operator_count: None,
                    register: None,
                    pending_action: None,
                },
                action: Action::Operate(Operator::Delete, Target::Motion(Motion::WordForward)),
            })
        );
        assert_eq!(parser.state.mode, Mode::Normal);
    }

    #[test]
    fn test_operator_pending_action() {
        let mut parser = VimParser::new();
        parser.state.mode = Mode::OperatorPending(Operator::Change);

        let result = handle(&mut parser, Operator::Change, 't');

        assert_eq!(result, ParseResult::Incomplete);
        assert_eq!(
            parser.state.context.pending_action,
            Some(PendingAction::TillForward)
        );
        assert_eq!(parser.state.mode, Mode::OperatorPending(Operator::Change));
    }

    #[test]
    fn test_operator_pending_with_count_cw_enters_insert() {
        let mut parser = VimParser::new();
        parser.state.mode = Mode::OperatorPending(Operator::Change);

        let result1 = handle(&mut parser, Operator::Change, '3');
        assert_eq!(result1, ParseResult::Incomplete);

        let result2 = handle(&mut parser, Operator::Change, 'w');
        assert_eq!(
            result2,
            ParseResult::Success(ParsedCommand {
                context: CommandContext {
                    count: Some(3),
                    operator_count: None,
                    register: None,
                    pending_action: None,
                },
                action: Action::Operate(Operator::Change, Target::Motion(Motion::WordForward)),
            })
        );
        assert_eq!(parser.state.mode, Mode::Insert);
    }
}

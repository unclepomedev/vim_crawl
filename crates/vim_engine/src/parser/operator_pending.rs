use crate::ast::action::Action;
use crate::ast::command::ParsedCommand;
use crate::ast::operator::Operator;
use crate::ast::target::Target;
use crate::error::ParseError;
use crate::parser::VimParser;
use crate::parser::mapping::{parse_motion, parse_operator, try_parse_count};
use crate::parser::result::ParseResult;
use crate::state::Mode;

fn combined_context(ctx: &crate::state::CommandContext) -> crate::state::CommandContext {
    let combined = match (ctx.operator_count, ctx.count) {
        (Some(a), Some(b)) => Some(a * b),
        (Some(a), None) => Some(a),
        (None, Some(b)) => Some(b),
        (None, None) => None,
    };
    crate::state::CommandContext {
        count: combined,
        operator_count: None,
        register: ctx.register,
    }
}

pub fn handle(parser: &mut VimParser, op: Operator, c: char) -> ParseResult {
    if try_parse_count(c, &mut parser.state.context) {
        return ParseResult::Incomplete;
    }

    if let Some(second_op) = parse_operator(c) {
        parser.state.mode = Mode::Normal;

        return if op == second_op {
            let action = Action::Operate(op, Target::Line);
            let command = ParsedCommand {
                context: combined_context(&parser.state.context),
                action,
            };
            ParseResult::Success(command)
        } else {
            ParseResult::Invalid(ParseError::UnknownCommand)
        };
    }

    if let Some(motion) = parse_motion(c) {
        parser.state.mode = Mode::Normal;
        let action = Action::Operate(op, Target::Motion(motion));
        let command = ParsedCommand {
            context: combined_context(&parser.state.context),
            action,
        };
        return ParseResult::Success(command);
    }

    parser.state.mode = Mode::Normal;
    ParseResult::Invalid(ParseError::InvalidMotion)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::motion::Motion;
    use crate::state::CommandContext;

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
        assert_eq!(parser.state.mode, Mode::Normal);
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
    fn test_operator_pending_valid_motion() {
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
                    register: None
                },
                action: Action::Operate(Operator::Delete, Target::Motion(Motion::WordForward)),
            })
        );
        assert_eq!(parser.state.mode, Mode::Normal);
    }
}

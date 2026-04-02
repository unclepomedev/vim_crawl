use crate::ast::action::Action;
use crate::ast::command::ParsedCommand;
use crate::ast::operator::Operator;
use crate::ast::target::Target;
use crate::error::ParseError;
use crate::parser::VimParser;
use crate::parser::mapping::parse_motion;
use crate::parser::result::ParseResult;
use crate::state::Mode;

pub fn handle(parser: &mut VimParser, op: Operator, c: char) -> ParseResult {
    if let Some(motion) = parse_motion(c) {
        parser.state.mode = Mode::Normal;

        let action = Action::Operate(op, Target::Motion(motion));
        let command = ParsedCommand {
            context: parser.state.context.clone(),
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
}

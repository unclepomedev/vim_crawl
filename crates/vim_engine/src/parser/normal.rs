use crate::ast::action::Action;
use crate::ast::command::ParsedCommand;
use crate::error::ParseError;
use crate::parser::VimParser;
use crate::parser::mapping::{parse_motion, parse_operator};
use crate::parser::result::ParseResult;
use crate::state::Mode;

pub fn handle(parser: &mut VimParser, c: char) -> ParseResult {
    if let Some(op) = parse_operator(c) {
        parser.state.mode = Mode::OperatorPending(op);
        return ParseResult::Incomplete;
    }

    if let Some(motion) = parse_motion(c) {
        let action = Action::Move(motion);
        let command = ParsedCommand {
            context: parser.state.context.clone(),
            action,
        };
        return ParseResult::Success(command);
    }

    ParseResult::Invalid(ParseError::UnknownCommand)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::motion::Motion;
    use crate::ast::operator::Operator;
    use crate::state::CommandContext;

    #[test]
    fn test_normal_mode_motions() {
        let mut parser = VimParser::new();

        let test_cases = vec![
            ('h', Motion::Left),
            ('j', Motion::Down),
            ('k', Motion::Up),
            ('l', Motion::Right),
            ('w', Motion::WordForward),
        ];

        for (input, expected_motion) in test_cases {
            let result = handle(&mut parser, input);
            assert_eq!(
                result,
                ParseResult::Success(ParsedCommand {
                    context: CommandContext::default(),
                    action: Action::Move(expected_motion),
                })
            );
        }
    }

    #[test]
    fn test_normal_mode_operator_transition() {
        let mut parser = VimParser::new();

        let result = handle(&mut parser, 'd');

        assert_eq!(result, ParseResult::Incomplete);
        assert_eq!(parser.state.mode, Mode::OperatorPending(Operator::Delete));
    }
}

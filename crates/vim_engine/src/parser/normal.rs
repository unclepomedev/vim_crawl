use crate::ast::action::Action;
use crate::ast::command::ParsedCommand;
use crate::error::ParseError;
use crate::parser::VimParser;
use crate::parser::mapping::{parse_motion, parse_operator, try_parse_count};
use crate::parser::result::ParseResult;
use crate::state::Mode;

pub fn handle(parser: &mut VimParser, c: char) -> ParseResult {
    if try_parse_count(c, &mut parser.state.context) {
        return ParseResult::Incomplete;
    }

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
    use crate::ast::target::Target;
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
            ('b', Motion::WordBackward),
            ('$', Motion::EndOfLine),
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

    #[test]
    fn test_count_motion() {
        let mut parser = VimParser::new();
        assert_eq!(parser.feed('3'), ParseResult::Incomplete);
        let result = parser.feed('w');

        if let ParseResult::Success(cmd) = result {
            assert_eq!(cmd.context.count, Some(3));
            assert_eq!(cmd.action, Action::Move(Motion::WordForward));
        } else {
            panic!("Expected Success");
        }
        assert_eq!(parser.state.context.count, None);
    }

    #[test]
    fn test_count_line_wise() {
        let mut parser = VimParser::new();
        parser.feed('2');
        parser.feed('d');
        let result = parser.feed('d');

        if let ParseResult::Success(cmd) = result {
            assert_eq!(cmd.context.count, Some(2));
            assert_eq!(cmd.action, Action::Operate(Operator::Delete, Target::Line));
        } else {
            panic!("Expected Success");
        }
    }

    #[test]
    fn test_zero_as_motion() {
        let mut parser = VimParser::new();
        let result = parser.feed('0');

        assert_eq!(
            result,
            ParseResult::Success(ParsedCommand {
                context: CommandContext::default(),
                action: Action::Move(Motion::StartOfLine),
            })
        );
        assert_eq!(parser.state.context.count, None);
    }

    #[test]
    fn test_zero_inside_count() {
        let mut parser = VimParser::new();
        parser.feed('1');
        let result = parser.feed('0');

        assert_eq!(result, ParseResult::Incomplete);
        assert_eq!(parser.state.context.count, Some(10));
    }

    #[test]
    fn test_multi_digit_count() {
        let mut parser = VimParser::new();
        parser.feed('1');
        parser.feed('2');
        parser.feed('3');
        let result = parser.feed('w');

        if let ParseResult::Success(cmd) = result {
            assert_eq!(cmd.context.count, Some(123));
            assert_eq!(cmd.action, Action::Move(Motion::WordForward));
        } else {
            panic!("Expected Success");
        }
    }
}

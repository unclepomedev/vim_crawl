use crate::ast::action::Action;
use crate::ast::command::ParsedCommand;
use crate::ast::motion::Motion;
use crate::error::ParseError;
use crate::parser::VimParser;
use crate::parser::result::ParseResult;

pub fn handle(parser: &mut VimParser, c: char) -> ParseResult {
    let motion = match c {
        'h' => Motion::Left,
        'j' => Motion::Down,
        'k' => Motion::Up,
        'l' => Motion::Right,
        _ => return ParseResult::Invalid(ParseError::UnknownCommand),
    };

    let action = Action::Move(motion);
    let command = ParsedCommand {
        context: parser.state.context.clone(),
        action,
    };

    ParseResult::Success(command)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::CommandContext;

    #[test]
    fn test_normal_mode_hjkl() {
        let mut parser = VimParser::new();

        let test_cases = vec![
            ('h', Motion::Left),
            ('j', Motion::Down),
            ('k', Motion::Up),
            ('l', Motion::Right),
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
    fn test_normal_mode_invalid_command() {
        let mut parser = VimParser::new();
        let result = handle(&mut parser, 'z');
        assert_eq!(result, ParseResult::Invalid(ParseError::UnknownCommand));
    }
}

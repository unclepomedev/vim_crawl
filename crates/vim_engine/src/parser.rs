use crate::error::ParseError;
use crate::parser::result::ParseResult;
use crate::state::{EditorState, Mode, PendingAction};

pub mod insert;
pub mod mapping;
pub mod normal;
pub mod operator_pending;
pub mod result;
pub mod visual;

#[derive(Default)]
pub struct VimParser {
    pub state: EditorState,
}

impl VimParser {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn feed(&mut self, c: char) -> ParseResult {
        let result = if let Some(pending) = self.state.context.pending_action.take() {
            self.resolve_pending(pending, c)
        } else {
            self.route_input(c)
        };

        match result {
            ParseResult::Success(_) | ParseResult::Invalid(_) => {
                self.state.context.reset();
            }
            ParseResult::Incomplete => {}
        }

        result
    }

    fn resolve_pending(&mut self, pending: PendingAction, c: char) -> ParseResult {
        if let Some(motion) = mapping::parse_pending_motion(pending, c) {
            return match self.state.mode {
                Mode::Normal => normal::handle_motion(self, motion),
                Mode::OperatorPending(op) => operator_pending::handle_motion(self, op, motion),
                Mode::Visual => unreachable!("Visual mode does not yet support pending actions"),
                Mode::Insert => ParseResult::Invalid(ParseError::UnknownCommand),
            };
        }

        if let Some(text_obj) = mapping::parse_text_object(pending, c) {
            return match self.state.mode {
                Mode::OperatorPending(op) => {
                    operator_pending::handle_text_object(self, op, text_obj)
                }
                Mode::Visual => unreachable!("Visual mode does not yet support text objects"),
                _ => ParseResult::Invalid(ParseError::UnknownCommand),
            };
        }

        self.state.mode = Mode::Normal;
        ParseResult::Invalid(ParseError::InvalidMotion)
    }

    fn route_input(&mut self, c: char) -> ParseResult {
        match self.state.mode {
            Mode::Normal => normal::handle(self, c),
            Mode::Insert => insert::handle(self, c),
            Mode::Visual => visual::handle(self, c),
            Mode::OperatorPending(op) => operator_pending::handle(self, op, c),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::action::Action;
    use crate::ast::command::ParsedCommand;
    use crate::ast::motion::Motion;
    use crate::ast::operator::Operator;
    use crate::ast::target::Target;
    use crate::ast::text_object::TextObject;
    use crate::state::CommandContext;

    #[test]
    fn test_parser_feed_pending_motion() {
        let mut parser = VimParser::new();

        let res1 = parser.feed('f');
        assert_eq!(res1, ParseResult::Incomplete);
        assert_eq!(
            parser.state.context.pending_action,
            Some(PendingAction::FindForward)
        );

        let res2 = parser.feed('x');
        assert_eq!(
            res2,
            ParseResult::Success(ParsedCommand {
                context: CommandContext::default(),
                action: Action::Move(Motion::FindForward('x')),
            })
        );
        assert_eq!(parser.state.context.pending_action, None);
        assert_eq!(parser.state.mode, Mode::Normal);
    }

    #[test]
    fn test_parser_feed_text_object() {
        let mut parser = VimParser::new();

        let res1 = parser.feed('d');
        assert_eq!(res1, ParseResult::Incomplete);
        assert_eq!(parser.state.mode, Mode::OperatorPending(Operator::Delete));

        let res2 = parser.feed('i');
        assert_eq!(res2, ParseResult::Incomplete);
        assert_eq!(
            parser.state.context.pending_action,
            Some(PendingAction::Inner)
        );

        let res3 = parser.feed('w');
        assert_eq!(
            res3,
            ParseResult::Success(ParsedCommand {
                context: CommandContext::default(),
                action: Action::Operate(
                    Operator::Delete,
                    Target::TextObject(TextObject::InnerWord)
                ),
            })
        );
        assert_eq!(parser.state.context.pending_action, None);
        assert_eq!(parser.state.mode, Mode::Normal);
    }

    #[test]
    fn test_parser_feed_invalid_text_object() {
        let mut parser = VimParser::new();

        parser.feed('d');
        parser.feed('i');
        let res3 = parser.feed('z');

        assert_eq!(res3, ParseResult::Invalid(ParseError::InvalidMotion));
        assert_eq!(parser.state.context.pending_action, None);
        assert_eq!(parser.state.mode, Mode::Normal);
    }

    #[test]
    fn test_parser_feed_change_text_object() {
        let mut parser = VimParser::new();

        parser.feed('c');
        parser.feed('a');
        let res3 = parser.feed('w');

        assert_eq!(
            res3,
            ParseResult::Success(ParsedCommand {
                context: CommandContext::default(),
                action: Action::Operate(Operator::Change, Target::TextObject(TextObject::AWord)),
            })
        );
        assert_eq!(parser.state.mode, Mode::Insert);
    }
}

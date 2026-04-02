//! The core parsing engine for Vim commands.
//!
//! This module implements the finite state machine responsible for translating
//! sequential character inputs into structured Vim commands.
//!
//! # Internal Architecture
//!
//! This section is for maintainers of the `vim_engine` crate.
//!
//! * **State Machine**: Maintains the current `Mode` (e.g., Normal, OperatorPending, Insert)
//!   and a `CommandContext` (accumulated counts, registers, and pending character requests).
//! * **Routing**: The `VimParser::feed` method acts as a router, dispatching inputs to the appropriate
//!   sub-module (`normal`, `operator_pending`, etc.) based on the current state.
//! * **Context Isolation**: When a command resolves (either as `Success` or `Invalid`), the
//!   parser automatically resets its temporary context (counts, pending actions) while preserving
//!   the appropriate mode transitions.

use crate::error::ParseError;
use crate::parser::result::ParseResult;
use crate::state::{EditorState, Mode, PendingAction};
use modes::{insert, normal, operator_pending, visual};

pub mod mapping;
pub mod modes;
pub mod result;
pub mod validation;

/// A stateful parser that translates sequential character inputs into Vim commands.
///
/// `VimParser` acts as a finite state machine, interpreting keystrokes based on standard
/// Vim grammar. It handles basic motions, operators, counts (multipliers), text objects,
/// and pending actions (such as waiting for a character after `f` or `i`).
///
/// The parser is designed to be completely decoupled from the game engine or UI. It strictly
/// focuses on semantic evaluation—taking raw characters via the [`feed`](Self::feed) method
/// and outputting a structured Abstract Syntax Tree (AST) wrapped in a [`ParseResult`].
///
/// # Examples
///
/// ```
/// use vim_engine::parser::VimParser;
/// use vim_engine::parser::result::ParseResult;
///
/// let mut parser = VimParser::new();
///
/// // Input: '3' (Count)
/// assert_eq!(parser.feed('3'), ParseResult::Incomplete);
///
/// // Input: 'd' (Operator)
/// assert_eq!(parser.feed('d'), ParseResult::Incomplete);
///
/// // Input: 'w' (Motion) -> Resolves to "Delete 3 Words"
/// if let ParseResult::Success(cmd) = parser.feed('w') {
///     // `cmd` contains the AST for 3 * Delete(WordForward).
///     // The parser automatically resets its context and returns to Normal mode.
/// }
/// ```
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
        if pending == PendingAction::Register {
            return if validation::is_valid_register(c) {
                self.state.context.register = Some(c);
                ParseResult::Incomplete
            } else {
                self.state.mode = Mode::Normal;
                ParseResult::Invalid(ParseError::UnknownCommand)
            };
        }

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

// Unit Tests ======================================================================================
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

    #[test]
    fn test_parser_count_operator_motion() {
        let mut parser = VimParser::new();

        assert_eq!(parser.feed('3'), ParseResult::Incomplete);
        assert_eq!(parser.feed('d'), ParseResult::Incomplete);

        if let ParseResult::Success(cmd) = parser.feed('w') {
            assert_eq!(cmd.context.count, Some(3));
            assert_eq!(
                cmd.action,
                Action::Operate(Operator::Delete, Target::Motion(Motion::WordForward))
            );
        } else {
            panic!("Expected Success result for '3dw' sequence");
        }
    }

    #[test]
    fn test_parser_feed_register_and_operator() {
        let mut parser = VimParser::new();

        let res1 = parser.feed('"');
        assert_eq!(res1, ParseResult::Incomplete);
        assert_eq!(
            parser.state.context.pending_action,
            Some(PendingAction::Register)
        );

        let res2 = parser.feed('+');
        assert_eq!(res2, ParseResult::Incomplete);
        assert_eq!(parser.state.context.pending_action, None);
        assert_eq!(parser.state.context.register, Some('+'));

        let res3 = parser.feed('d');
        assert_eq!(res3, ParseResult::Incomplete);
        assert_eq!(parser.state.mode, Mode::OperatorPending(Operator::Delete));

        let res4 = parser.feed('w');
        assert_eq!(
            res4,
            ParseResult::Success(ParsedCommand {
                context: CommandContext {
                    count: None,
                    operator_count: None,
                    register: Some('+'),
                    pending_action: None,
                },
                action: Action::Operate(Operator::Delete, Target::Motion(Motion::WordForward)),
            })
        );

        assert_eq!(parser.state.context.register, None);
        assert_eq!(parser.state.mode, Mode::Normal);
    }

    #[test]
    fn test_parser_feed_invalid_register() {
        let mut parser = VimParser::new();
        parser.feed('"');
        let res = parser.feed(' ');

        assert_eq!(res, ParseResult::Invalid(ParseError::UnknownCommand));
        assert_eq!(parser.state.context.pending_action, None);
        assert_eq!(parser.state.context.register, None);
        assert_eq!(parser.state.mode, Mode::Normal);
    }
}

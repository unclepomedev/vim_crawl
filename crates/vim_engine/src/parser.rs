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

use crate::ast::action::Action;
use crate::ast::command::ParsedCommand;
use crate::error::ParseError;
use crate::parser::key::Key;
use crate::parser::result::ParseResult;
use crate::state::CommandContext;
use crate::state::{EditorState, Mode, PendingAction};
use modes::{insert, normal, operator_pending, visual};

pub mod key;
pub mod mapping;
pub mod modes;
pub mod result;
pub mod validation;

/// A stateful parser that translates sequential keystrokes into Vim commands.
///
/// `VimParser` acts as a finite state machine, interpreting keys based on standard
/// Vim grammar. It handles basic motions, operators, counts (multipliers), text objects,
/// and pending actions.
///
/// # Input Encoding Contract
///
/// The `feed` method expects inputs as variants of the `Key` enum. Clients must map
/// physical key events to the `Key` variants before passing them to `feed`.
///
/// # Examples
///
/// ```
/// use vim_engine::parser::VimParser;
/// use vim_engine::parser::result::ParseResult;
/// use vim_engine::parser::key::Key;
///
/// let mut parser = VimParser::new();
///
/// assert_eq!(parser.feed(Key::Char('3')), ParseResult::Incomplete);
///
/// if let ParseResult::Success(cmd) = parser.feed(Key::Char('u')) {
///     assert_eq!(cmd.context.count, Some(3));
///     assert_eq!(cmd.action, vim_engine::ast::action::Action::Undo);
/// } else {
///     panic!("Expected Success for '3u'");
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

    pub fn feed(&mut self, key: Key) -> ParseResult {
        if mapping::is_cancel_key(key) {
            self.state.mode = Mode::Normal;
            self.state.context.reset();
            return ParseResult::Success(ParsedCommand {
                context: CommandContext::default(),
                action: Action::Cancel,
            });
        }

        let result = if let Some(pending) = self.state.context.pending_action.take() {
            self.resolve_pending(pending, key)
        } else {
            self.route_input(key)
        };

        match result {
            ParseResult::Success(_) | ParseResult::Invalid(_) => {
                self.state.context.reset();
            }
            ParseResult::Incomplete => {}
        }

        result
    }

    fn resolve_pending(&mut self, pending: PendingAction, key: Key) -> ParseResult {
        if pending == PendingAction::Register {
            if let Key::Char(c) = key
                && validation::is_valid_register(c)
            {
                self.state.context.register = Some(c);
                return ParseResult::Incomplete;
            }
            self.state.mode = Mode::Normal;
            return ParseResult::Invalid(ParseError::UnknownCommand);
        }

        if let Some(motion) = mapping::parse_pending_motion(pending, key) {
            return match self.state.mode {
                Mode::Normal => normal::handle_motion(self, motion),
                Mode::OperatorPending(op) => operator_pending::handle_motion(self, op, motion),
                Mode::Visual => unreachable!("Visual mode does not yet support pending actions"),
                Mode::Insert => ParseResult::Invalid(ParseError::UnknownCommand),
            };
        }

        if let Some(text_obj) = mapping::parse_text_object(pending, key) {
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

    fn route_input(&mut self, key: Key) -> ParseResult {
        match self.state.mode {
            Mode::Normal => normal::handle(self, key),
            Mode::Insert => insert::handle(self, key),
            Mode::Visual => visual::handle(self, key),
            Mode::OperatorPending(op) => operator_pending::handle(self, op, key),
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

        let res1 = parser.feed(Key::Char('f'));
        assert_eq!(res1, ParseResult::Incomplete);
        assert_eq!(
            parser.state.context.pending_action,
            Some(PendingAction::FindForward)
        );

        let res2 = parser.feed(Key::Char('x'));
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

        let res1 = parser.feed(Key::Char('d'));
        assert_eq!(res1, ParseResult::Incomplete);
        assert_eq!(parser.state.mode, Mode::OperatorPending(Operator::Delete));

        let res2 = parser.feed(Key::Char('i'));
        assert_eq!(res2, ParseResult::Incomplete);
        assert_eq!(
            parser.state.context.pending_action,
            Some(PendingAction::Inner)
        );

        let res3 = parser.feed(Key::Char('w'));
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

        parser.feed(Key::Char('d'));
        parser.feed(Key::Char('i'));
        let res3 = parser.feed(Key::Char('z'));

        assert_eq!(res3, ParseResult::Invalid(ParseError::InvalidMotion));
        assert_eq!(parser.state.context.pending_action, None);
        assert_eq!(parser.state.mode, Mode::Normal);
    }

    #[test]
    fn test_parser_feed_change_text_object() {
        let mut parser = VimParser::new();

        parser.feed(Key::Char('c'));
        parser.feed(Key::Char('a'));
        let res3 = parser.feed(Key::Char('w'));

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

        assert_eq!(parser.feed(Key::Char('3')), ParseResult::Incomplete);
        assert_eq!(parser.feed(Key::Char('d')), ParseResult::Incomplete);

        if let ParseResult::Success(cmd) = parser.feed(Key::Char('w')) {
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

        let res1 = parser.feed(Key::Char('"'));
        assert_eq!(res1, ParseResult::Incomplete);
        assert_eq!(
            parser.state.context.pending_action,
            Some(PendingAction::Register)
        );

        let res2 = parser.feed(Key::Char('+'));
        assert_eq!(res2, ParseResult::Incomplete);
        assert_eq!(parser.state.context.pending_action, None);
        assert_eq!(parser.state.context.register, Some('+'));

        let res3 = parser.feed(Key::Char('d'));
        assert_eq!(res3, ParseResult::Incomplete);
        assert_eq!(parser.state.mode, Mode::OperatorPending(Operator::Delete));

        let res4 = parser.feed(Key::Char('w'));
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
        parser.feed(Key::Char('"'));
        let res = parser.feed(Key::Char(' '));

        assert_eq!(res, ParseResult::Invalid(ParseError::UnknownCommand));
        assert_eq!(parser.state.context.pending_action, None);
        assert_eq!(parser.state.context.register, None);
        assert_eq!(parser.state.mode, Mode::Normal);
    }

    #[test]
    fn test_parser_cancel_from_insert_mode() {
        let mut parser = VimParser::new();

        parser.feed(Key::Char('i'));
        assert_eq!(parser.state.mode, Mode::Insert);

        let res = parser.feed(Key::Esc);

        assert_eq!(
            res,
            ParseResult::Success(ParsedCommand {
                context: CommandContext::default(),
                action: Action::Cancel,
            })
        );
        assert_eq!(parser.state.mode, Mode::Normal);
    }

    #[test]
    fn test_parser_cancel_pending_state() {
        let mut parser = VimParser::new();

        parser.feed(Key::Char('3'));
        parser.feed(Key::Char('d'));
        assert_eq!(parser.state.mode, Mode::OperatorPending(Operator::Delete));

        assert_eq!(parser.state.context.operator_count, Some(3));

        let res = parser.feed(Key::Ctrl('c'));

        assert_eq!(
            res,
            ParseResult::Success(ParsedCommand {
                context: CommandContext::default(),
                action: Action::Cancel,
            })
        );
        assert_eq!(parser.state.mode, Mode::Normal);
        assert_eq!(parser.state.context.count, None);
        assert_eq!(parser.state.context.operator_count, None);
    }
}

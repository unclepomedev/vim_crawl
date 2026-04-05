//! Map a character to an AST node.

use crate::ast::action::{Action, InsertCommand, PasteDir};
use crate::ast::motion::Motion;
use crate::ast::operator::Operator;
use crate::ast::target::Target;
use crate::ast::text_object::TextObject;
use crate::parser::key::Key;
use crate::state::{CommandContext, PendingAction};

pub fn parse_motion(key: Key) -> Option<Motion> {
    match key {
        Key::Char('h') => Some(Motion::Left),
        Key::Char('j') => Some(Motion::Down),
        Key::Char('k') => Some(Motion::Up),
        Key::Char('l') => Some(Motion::Right),
        Key::Char('w') => Some(Motion::WordForward),
        Key::Char('b') => Some(Motion::WordBackward),
        Key::Char('e') => Some(Motion::WordEndForward),
        Key::Char('$') => Some(Motion::EndOfLine),
        Key::Char('0') => Some(Motion::StartOfLine),
        Key::Char('^') => Some(Motion::FirstNonBlank),
        Key::Char('G') => Some(Motion::GotoLine),
        Key::Char('%') => Some(Motion::MatchPairs),
        _ => None,
    }
}

pub fn parse_operator(key: Key) -> Option<Operator> {
    match key {
        Key::Char('d') => Some(Operator::Delete),
        Key::Char('y') => Some(Operator::Yank),
        Key::Char('c') => Some(Operator::Change),
        Key::Char('>') => Some(Operator::Indent),
        Key::Char('<') => Some(Operator::Outdent),
        Key::Char('=') => Some(Operator::Format),
        _ => None,
    }
}

pub fn parse_pending_action(key: Key) -> Option<PendingAction> {
    match key {
        Key::Char('f') => Some(PendingAction::FindForward),
        Key::Char('F') => Some(PendingAction::FindBackward),
        Key::Char('t') => Some(PendingAction::TillForward),
        Key::Char('T') => Some(PendingAction::TillBackward),
        Key::Char('"') => Some(PendingAction::Register),
        Key::Char('r') => Some(PendingAction::ReplaceChar),
        Key::Char('g') => Some(PendingAction::GCommand),
        _ => None,
    }
}

pub fn parse_text_object_modifier(key: Key) -> Option<PendingAction> {
    match key {
        Key::Char('i') => Some(PendingAction::Inner),
        Key::Char('a') => Some(PendingAction::Around),
        _ => None,
    }
}

pub fn parse_pending_motion(pending: PendingAction, key: Key) -> Option<Motion> {
    let c = match key {
        Key::Char(c) => c,
        _ => return None,
    };

    match pending {
        PendingAction::FindForward => Some(Motion::FindForward(c)),
        PendingAction::FindBackward => Some(Motion::FindBackward(c)),
        PendingAction::TillForward => Some(Motion::TillForward(c)),
        PendingAction::TillBackward => Some(Motion::TillBackward(c)),
        _ => None,
    }
}

pub fn parse_g_command(key: Key) -> Option<Action> {
    match key {
        Key::Char('g') => Some(Action::Move(Motion::GotoLine)),
        Key::Char('e') => Some(Action::Move(Motion::WordEndBackward)),
        Key::Char('i') => Some(Action::EnterInsert(InsertCommand::InsertLast)),
        _ => None,
    }
}

pub fn parse_text_object(pending: PendingAction, key: Key) -> Option<TextObject> {
    match (pending, key) {
        (PendingAction::Inner, Key::Char('w')) => Some(TextObject::InnerWord),
        (PendingAction::Around, Key::Char('w')) => Some(TextObject::AWord),
        _ => None,
    }
}

/// If the input characters are valid as part of the Count, update the context and return true.
/// If it's invalid, return false.
pub fn try_parse_count(key: Key, context: &mut CommandContext) -> bool {
    let c = match key {
        Key::Char(c) => c,
        _ => return false,
    };

    if c == '0' && context.count.is_none() {
        return false;
    }

    if c.is_ascii_digit() {
        let digit = c.to_digit(10).unwrap() as usize;
        let current = context.count.unwrap_or(0);
        context.count = current
            .checked_mul(10)
            .and_then(|v| v.checked_add(digit))
            .or(context.count); // Preserve previous count on overflow
        true
    } else {
        false
    }
}

pub fn parse_standalone_action(key: Key) -> Option<Action> {
    match key {
        Key::Char('u') => Some(Action::Undo),
        Key::Ctrl('r') => Some(Action::Redo),
        Key::Char('p') => Some(Action::Paste(PasteDir::After)),
        Key::Char('P') => Some(Action::Paste(PasteDir::Before)),
        Key::Char('J') => Some(Action::JoinLines),
        Key::Char('~') => Some(Action::ToggleCase),
        Key::Char('i') => Some(Action::EnterInsert(InsertCommand::Insert)),
        Key::Char('a') => Some(Action::EnterInsert(InsertCommand::Append)),
        Key::Char('I') => Some(Action::EnterInsert(InsertCommand::InsertAtLineStart)),
        Key::Char('A') => Some(Action::EnterInsert(InsertCommand::AppendAtLineEnd)),
        Key::Char('o') => Some(Action::EnterInsert(InsertCommand::OpenLineBelow)),
        Key::Char('O') => Some(Action::EnterInsert(InsertCommand::OpenLineAbove)),
        Key::Char('v') => Some(Action::EnterVisual),
        _ => None,
    }
}

pub fn parse_alias(key: Key) -> Option<Action> {
    match key {
        // x -> dl
        Key::Char('x') => Some(Action::Operate(
            Operator::Delete,
            Target::Motion(Motion::Right),
        )),
        // X -> dh
        Key::Char('X') => Some(Action::Operate(
            Operator::Delete,
            Target::Motion(Motion::Left),
        )),
        // s -> cl
        Key::Char('s') => Some(Action::Operate(
            Operator::Change,
            Target::Motion(Motion::Right),
        )),
        // S -> cc
        Key::Char('S') => Some(Action::Operate(Operator::Change, Target::Line)),
        // D -> d$
        Key::Char('D') => Some(Action::Operate(
            Operator::Delete,
            Target::Motion(Motion::EndOfLine),
        )),
        // C -> c$
        Key::Char('C') => Some(Action::Operate(
            Operator::Change,
            Target::Motion(Motion::EndOfLine),
        )),
        // Y -> yy
        Key::Char('Y') => Some(Action::Operate(Operator::Yank, Target::Line)),
        _ => None,
    }
}

pub fn is_cancel_key(key: Key) -> bool {
    matches!(key, Key::Esc | Key::Ctrl('c'))
}

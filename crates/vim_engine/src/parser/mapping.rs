//! Map a character to an AST node.

use crate::ast::motion::Motion;
use crate::ast::operator::Operator;
use crate::ast::text_object::TextObject;
use crate::state::{CommandContext, PendingAction};

pub fn parse_motion(c: char) -> Option<Motion> {
    match c {
        'h' => Some(Motion::Left),
        'j' => Some(Motion::Down),
        'k' => Some(Motion::Up),
        'l' => Some(Motion::Right),
        'w' => Some(Motion::WordForward),
        'b' => Some(Motion::WordBackward),
        '$' => Some(Motion::EndOfLine),
        '0' => Some(Motion::StartOfLine),
        _ => None,
    }
}

pub fn parse_operator(c: char) -> Option<Operator> {
    match c {
        'd' => Some(Operator::Delete),
        'y' => Some(Operator::Yank),
        'c' => Some(Operator::Change),
        _ => None,
    }
}

pub fn parse_pending_action(c: char) -> Option<PendingAction> {
    match c {
        'f' => Some(PendingAction::FindForward),
        'F' => Some(PendingAction::FindBackward),
        't' => Some(PendingAction::TillForward),
        'T' => Some(PendingAction::TillBackward),
        '"' => Some(PendingAction::Register),
        _ => None,
    }
}

pub fn parse_text_object_modifier(c: char) -> Option<PendingAction> {
    match c {
        'i' => Some(PendingAction::Inner),
        'a' => Some(PendingAction::Around),
        _ => None,
    }
}

pub fn parse_pending_motion(pending: PendingAction, c: char) -> Option<Motion> {
    match pending {
        PendingAction::FindForward => Some(Motion::FindForward(c)),
        PendingAction::FindBackward => Some(Motion::FindBackward(c)),
        PendingAction::TillForward => Some(Motion::TillForward(c)),
        PendingAction::TillBackward => Some(Motion::TillBackward(c)),
        _ => None,
    }
}

pub fn parse_text_object(pending: PendingAction, c: char) -> Option<TextObject> {
    match (pending, c) {
        (PendingAction::Inner, 'w') => Some(TextObject::InnerWord),
        (PendingAction::Around, 'w') => Some(TextObject::AWord),
        _ => None,
    }
}

/// If the input characters are valid as part of the Count, update the context and return true.
/// If it's invalid, return false.
pub fn try_parse_count(c: char, context: &mut CommandContext) -> bool {
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

/// Returns true if the character is a valid Vim register name.
/// Valid registers include:
/// - Named / Append: 'a'..='z', 'A'..='Z'
/// - Numbered: '0'..='9'
/// - Special: '"' (unnamed), '-' (small delete), '_' (black hole),
///   '+', '*' (clipboard), '=', '/', ':', '.', '%', '#'
pub fn is_valid_register(c: char) -> bool {
    c.is_ascii_alphanumeric()
        || matches!(
            c,
            '"' | '-' | '_' | '+' | '*' | '=' | '/' | ':' | '.' | '%' | '#'
        )
}

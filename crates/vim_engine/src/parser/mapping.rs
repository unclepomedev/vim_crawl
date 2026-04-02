//! Map a character to an AST node.

use crate::ast::motion::Motion;
use crate::ast::operator::Operator;

pub fn parse_motion(c: char) -> Option<Motion> {
    match c {
        'h' => Some(Motion::Left),
        'j' => Some(Motion::Down),
        'k' => Some(Motion::Up),
        'l' => Some(Motion::Right),
        'w' => Some(Motion::WordForward),
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

use crate::ast::motion::Motion;
use crate::ast::operator::Operator;
use crate::ast::target::Target;

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub enum InsertCommand {
    Insert,            // i
    Append,            // a
    InsertAtLineStart, // I
    AppendAtLineEnd,   // A
    OpenLineBelow,     // o
    OpenLineAbove,     // O
    InsertLast,        // gi
}

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub enum PasteDir {
    Before, // P
    After,  // p
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Action {
    Move(Motion),
    Operate(Operator, Target),
    EnterInsert(InsertCommand),
    Insert(String),
    Backspace,
    Replace(char), // r{char}
    Paste(PasteDir),
    JoinLines,  // J
    ToggleCase, // ~
    MacroRecord(char),
    MacroExecute(char),
    Undo,
    Redo,
    EnterVisual,
    Cancel,
}

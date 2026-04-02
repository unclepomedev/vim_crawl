use crate::ast::motion::Motion;
use crate::ast::operator::Operator;
use crate::ast::target::Target;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Action {
    Move(Motion),
    Operate(Operator, Target),
    Insert(String),
    Backspace,
    MacroRecord(char),
    MacroExecute(char),
    Undo,
    Redo,
    EnterInsert,
    EnterVisual,
    Cancel,
}

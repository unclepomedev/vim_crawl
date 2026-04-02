#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub enum Operator {
    Delete,
    Yank,
    Change,
}

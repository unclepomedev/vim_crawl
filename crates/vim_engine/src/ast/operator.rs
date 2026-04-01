#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Operator {
    Delete,
    Yank,
    Change,
}

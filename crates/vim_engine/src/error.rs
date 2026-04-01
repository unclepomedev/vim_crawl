#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseError {
    UnknownCommand,
    InvalidMotion,
    InvalidRegister,
    MacroNotFound,
}

use std::error::Error;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseError {
    UnknownCommand,
    InvalidMotion,
    InvalidRegister,
    MacroNotFound,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownCommand => write!(f, "unknown command"),
            Self::InvalidMotion => write!(f, "invalid motion"),
            Self::InvalidRegister => write!(f, "invalid register"),
            Self::MacroNotFound => write!(f, "macro not found"),
        }
    }
}

impl Error for ParseError {}

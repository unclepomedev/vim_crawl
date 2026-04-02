use crate::ast::motion::Motion;
use crate::ast::text_object::TextObject;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Target {
    Motion(Motion),
    TextObject(TextObject),
    Line,
}

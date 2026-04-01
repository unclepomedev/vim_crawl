use crate::ast::action::Action;
use crate::state::CommandContext;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParsedCommand {
    pub context: CommandContext,
    pub action: Action,
}

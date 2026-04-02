use crate::ast::operator::Operator;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    Normal,
    Insert,
    Visual,
    OperatorPending(Operator),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PendingAction {
    FindForward,
    FindBackward,
    TillForward,
    TillBackward,
    Inner,
    Around,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct CommandContext {
    pub count: Option<usize>,
    pub operator_count: Option<usize>,
    pub register: Option<char>,
    pub pending_action: Option<PendingAction>,
}
impl CommandContext {
    pub fn reset(&mut self) {
        self.count = None;
        self.operator_count = None;
        self.register = None;
        self.pending_action = None;
    }
}

#[derive(Debug, Clone)]
pub struct EditorState {
    pub mode: Mode,
    pub context: CommandContext,
}

impl Default for EditorState {
    fn default() -> Self {
        Self {
            mode: Mode::Normal,
            context: CommandContext::default(),
        }
    }
}

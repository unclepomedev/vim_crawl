#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    Normal,
    Insert,
    Visual,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct CommandContext {
    pub count: Option<usize>,
    pub register: Option<char>,
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

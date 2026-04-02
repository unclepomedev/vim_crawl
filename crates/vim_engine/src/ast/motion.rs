#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Motion {
    Left,
    Right,
    Up,
    Down,
    WordForward,
    WordBackward,
    StartOfLine,
    EndOfLine,
    FindForward(char),
    FindBackward(char),
    TillForward(char),
    TillBackward(char),
}

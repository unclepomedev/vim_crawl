#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Motion {
    Left,
    Right,
    Up,
    Down,
    WordForward,
    WordBackward,
    WordEndForward,  // e
    WordEndBackward, // ge
    StartOfLine,     // 0
    FirstNonBlank,   // ^
    EndOfLine,       // $
    GotoLine,        // G, gg
    MatchPairs,      // %
    FindForward(char),
    FindBackward(char),
    TillForward(char),
    TillBackward(char),
}

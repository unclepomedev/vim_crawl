#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Key {
    Char(char), // e.g. 'a', 'D', '3', '$'
    Ctrl(char), // e.g. Key::Ctrl('r'), Key::Ctrl('d')
    Esc,
    Enter,
    Backspace,
    Up,
    Down,
    Left,
    Right,
}

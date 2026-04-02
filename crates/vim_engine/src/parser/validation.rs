/// Returns true if the character is a valid Vim register name.
/// Valid registers include:
/// - Named / Append: 'a'..='z', 'A'..='Z'
/// - Numbered: '0'..='9'
/// - Special: '"' (unnamed), '-' (small delete), '_' (black hole),
///   '+', '*' (clipboard), '~' (drop), '=', '/', ':', '.', '%', '#'
pub fn is_valid_register(c: char) -> bool {
    c.is_ascii_alphanumeric()
        || matches!(
            c,
            '"' | '-' | '_' | '+' | '*' | '~' | '=' | '/' | ':' | '.' | '%' | '#'
        )
}

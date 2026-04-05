use crate::state::vim::VimState;

pub fn handle_insert(vim_state: &mut VimState, text: String) {
    vim_state.buffer.push_str(&text);
}

pub fn handle_backspace(vim_state: &mut VimState) {
    vim_state.buffer.pop();
}

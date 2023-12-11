use tui::widgets::ListState;

pub fn is_selected(list_state: &ListState) -> bool {
    match list_state.selected() {
        Some(_) => true,
        None => false,
    }
}

pub fn init_index_table(list_state: &mut ListState, len: usize) {
    let is_selected = is_selected(list_state);

    if len == 0 {
        list_state.select(None);
        return;
    }

    if !is_selected {
        list_state.select(Some(0))
    } else {
        list_state.select(None);
    }
}

pub fn move_down(list_state: &mut ListState, len: usize) {
    if len == 0 {
        list_state.select(None);
        return;
    }

    let i = match list_state.selected() {
        Some(i) => {
            if i >= len - 1 { Some(len - 1) } else { Some(i + 1) }
        }
        None => Some(0),
    };
    list_state.select(i);
}

pub fn move_up(list_state: &mut ListState, len: usize) {
    if len == 0 {
        list_state.select(None);
        return;
    }

    let i = match list_state.selected() {
        Some(i) => {
            if i == 0 { Some(i) } else { Some(i - 1) }
        }
        None => Some(0),
    };
    list_state.select(i);
}

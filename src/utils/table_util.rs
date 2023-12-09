use tui::widgets::TableState;

pub fn is_selected(table_state: &TableState) -> bool {
    match table_state.selected() {
        Some(_) => true,
        None => false,
    }
}

pub fn init_index_table(table_state: &mut TableState, len: usize) {
    let is_selected = is_selected(table_state);

    if !is_selected && len > 0 {
        table_state.select(Some(0))
    }
}

pub fn move_down(table_state: &mut TableState, len: usize) {
    if len == 0 {
        table_state.select(None);
    }

    let i = match table_state.selected() {
        Some(i) => {
            if i >= len - 1 { Some(i) } else { Some(i + 1) }
        }
        None => None,
    };
    table_state.select(i);
}

pub fn move_up(table_state: &mut TableState, len: usize) {
    if len == 0 {
        table_state.select(None);
    }

    let i = match table_state.selected() {
        Some(i) => {
            if i == 0 { Some(i) } else { Some(i - 1) }
        }
        None => None,
    };
    table_state.select(i);
}

use super::dir_filter_state::DirFilterState;

const HELP: &'static str = "| ESC~Back | ↑ Up | ↓ Down | [~Prev |";

pub struct DirFilterComponent {
    pub state: DirFilterState,
}

impl DirFilterComponent {
    pub fn init() -> Self {
        DirFilterComponent {
            state: DirFilterState::init(),
        }
    }

    pub fn get_helper_text(&self) -> &'static str {
        HELP
    }
}

use super::dir_filter_state::DirFilterState;


const HELP: &'static str = "| ↑ Up | ↓ Down | TAB~Next | BACKTAB~Prev";

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

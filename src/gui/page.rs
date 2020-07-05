use iced::{button, scrollable};

pub(crate) enum PageModel {
    FirstQuestion,
    MiddleQuestion,
    LastQuestion,
    Results,
}

pub(crate) struct State {
    pub(crate) back_button: button::State,
    pub(crate) next_button: button::State,
    pub(crate) finish_button: button::State,
    pub(crate) restart_button: button::State,
    pub(crate) questions_labels: Vec<button::State>,
    pub(crate) scroll: scrollable::State,
}

impl State {
    pub(crate) fn new(questions_count: usize) -> Self {
        Self {
            back_button: button::State::new(),
            next_button: button::State::new(),
            finish_button: button::State::new(),
            restart_button: button::State::new(),
            questions_labels: vec![button::State::new(); questions_count],
            scroll: scrollable::State::new(),
        }
    }
}

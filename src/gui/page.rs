use iced::{button, scrollable};
use md_questions::Questions;

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
    pub(crate) selected_answers: Vec<Option<usize>>,
    pub(crate) questions: Questions,
    pub(crate) page_idx: usize,
}

impl State {
    pub(crate) fn new(questions: Questions) -> Self {
        Self {
            back_button: button::State::new(),
            next_button: button::State::new(),
            finish_button: button::State::new(),
            restart_button: button::State::new(),
            questions_labels: vec![button::State::new(); questions.len()],
            scroll: scrollable::State::new(),
            selected_answers: vec![None; questions.len()],
            questions,
            page_idx: 0,
        }
    }
}

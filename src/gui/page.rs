use iced::button;

pub(crate) enum PageModel {
    FirstQuestion {
        back_button: button::State,
        next_button: button::State,
        questions_labels: Vec<button::State>,
    },
    MiddleQuestion {
        back_button: button::State,
        next_button: button::State,
        questions_labels: Vec<button::State>,
    },
    LastQuestion {
        back_button: button::State,
        finish_button: button::State,
        questions_labels: Vec<button::State>,
    },
    Results {
        back_button: button::State,
        restart_button: button::State,
        questions_labels: Vec<button::State>,
    },
}

pub(crate) fn first_question(questions_count: usize) -> PageModel {
    PageModel::FirstQuestion {
        back_button: button::State::new(),
        next_button: button::State::new(),
        questions_labels: vec![button::State::new(); questions_count],
    }
}

pub(crate) fn last_question(questions_count: usize) -> PageModel {
    PageModel::LastQuestion {
        back_button: button::State::new(),
        finish_button: button::State::new(),
        questions_labels: vec![button::State::new(); questions_count],
    }
}

pub(crate) fn middle_question(questions_count: usize) -> PageModel {
    PageModel::MiddleQuestion {
        back_button: button::State::new(),
        next_button: button::State::new(),
        questions_labels: vec![button::State::new(); questions_count],
    }
}

pub(crate) fn results(questions_count: usize) -> PageModel {
    PageModel::Results {
        back_button: button::State::new(),
        restart_button: button::State::new(),
        questions_labels: vec![button::State::new(); questions_count],
    }
}

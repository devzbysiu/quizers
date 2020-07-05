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

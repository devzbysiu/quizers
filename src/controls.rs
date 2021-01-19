use crate::quizers::Msg;
use crate::view::button;
use crate::view::Buttons;
use iced::button;

#[derive(Default)]
pub(crate) struct Controls {
    questions_count: usize,
    back_button: button::State,
    next_button: button::State,
    finish_button: button::State,
    restart_button: button::State,
}

impl Controls {
    pub(crate) fn new(questions_count: usize) -> Self {
        Self {
            questions_count,
            ..Self::default()
        }
    }

    pub(crate) fn ctrls(&mut self, page_idx: usize) -> Buttons<'_> {
        match page_idx {
            0 => {
                let back = button(&mut self.back_button, "Back");
                let next = button(&mut self.next_button, "Next").on_press(Msg::NextPressed);
                (back, next)
            }
            x if x == self.questions_count - 1 => {
                let back = button(&mut self.back_button, "Back");
                let finish = button(&mut self.finish_button, "Finish").on_press(Msg::ShowResults);
                (back, finish)
            }
            x if x == self.questions_count => {
                let back = button(&mut self.back_button, "Back");
                let restart =
                    button(&mut self.restart_button, "Restart").on_press(Msg::RestartPressed);
                (back, restart)
            }
            _ => {
                let back = button(&mut self.back_button, "Back").on_press(Msg::BackPressed);
                let next = button(&mut self.next_button, "Next").on_press(Msg::NextPressed);
                (back, next)
            }
        }
    }
}

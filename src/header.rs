use crate::quizers::Msg;
use crate::view::header_button;
use crate::view::Elem;
use iced::{button, Length, Row, Space};

#[derive(Default)]
pub(crate) struct Header {
    settings_button: button::State,
    go_back_button: button::State,
    restart_button: button::State,
    finish_button: button::State,
}

impl Header {
    pub(crate) fn new() -> Self {
        Self::default()
    }

    pub(crate) fn view(&mut self) -> Elem<'_> {
        let go_back_btn =
            header_button(&mut self.go_back_button, " < ").on_press(Msg::GoBackPressed);

        #[allow(unused_variables)] // TODO: remove this
        let settings_btn =
            header_button(&mut self.settings_button, "S").on_press(Msg::SettingsPressed);

        let finish_btn = header_button(&mut self.finish_button, "F").on_press(Msg::ShowResults);

        let restart_btn =
            header_button(&mut self.restart_button, "R").on_press(Msg::RestartPressed);

        Row::new()
            .padding(10)
            .push(go_back_btn)
            .push(Space::with_width(Length::Fill))
            .push(restart_btn)
            .push(Space::with_width(Length::Units(3)))
            .push(finish_btn)
            .push(Space::with_width(Length::Units(3)))
            // .push(settings_btn) TODO: add settings when ready
            .into()
    }
}

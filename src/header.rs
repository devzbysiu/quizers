use crate::quizers::Msg;
use crate::view::settings_button;
use crate::view::Elem;
use iced::{button, Length, Row, Space};

pub(crate) struct Header {
    settings_button: button::State,
    go_back_button: button::State,
    restart_button: button::State,
}

impl Header {
    pub(crate) fn new() -> Self {
        Self {
            settings_button: button::State::new(),
            go_back_button: button::State::new(),
            restart_button: button::State::new(),
        }
    }

    pub(crate) fn view(&mut self) -> Elem<'_> {
        let go_back_btn =
            settings_button(&mut self.go_back_button, "<").on_press(Msg::GoBackPressed);

        let settings_btn =
            settings_button(&mut self.settings_button, "S").on_press(Msg::SettingsPressed);

        let restart_btn =
            settings_button(&mut self.restart_button, "R").on_press(Msg::RestartPressed);

        Row::new()
            .padding(10)
            .push(go_back_btn)
            .push(Space::with_width(Length::Fill))
            .push(restart_btn)
            .push(Space::with_width(Length::Units(3)))
            .push(settings_btn)
            .into()
    }
}

use crate::helpers::settings_button;
use crate::quizers::{Elem, Msg};
use iced::{button, Length, Row, Space};

pub(crate) struct Header {
    settings_button: button::State,
    go_back_button: button::State,
}

impl Header {
    pub(crate) fn new() -> Self {
        Self {
            settings_button: button::State::new(),
            go_back_button: button::State::new(),
        }
    }

    pub(crate) fn view<'a>(&'a mut self) -> Elem<'a> {
        let go_back_button =
            settings_button(&mut self.go_back_button, "<").on_press(Msg::GoBackPressed);

        let settings_button =
            settings_button(&mut self.settings_button, "S").on_press(Msg::SettingsPressed);

        Row::new()
            .height(Length::FillPortion(1))
            .padding(10)
            .push(go_back_button)
            .push(Space::with_width(Length::Fill))
            .push(settings_button)
            .into()
    }
}

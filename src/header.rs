use crate::quizers::{Elem, Msg};
use crate::style;
use iced::{button, Button, HorizontalAlignment, Text};

pub(crate) struct Header {
    setting_button: button::State,
}

impl Header {
    pub(crate) fn new() -> Self {
        Self {
            setting_button: button::State::new(),
        }
    }

    pub(crate) fn view<'a>(&'a mut self) -> Elem<'a> {
        Button::new(
            &mut self.setting_button,
            Text::new("S").horizontal_alignment(HorizontalAlignment::Center),
        )
        .min_width(35)
        .on_press(Msg::SettingsPressed)
        .style(style::SettingsButton)
        .into()
    }
}

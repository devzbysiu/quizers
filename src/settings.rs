use crate::view::Elem;
use iced::Text;

pub(crate) struct Settings {}

impl Settings {
    pub(crate) fn new() -> Self {
        Self {}
    }

    pub(crate) fn view<'a>(&mut self) -> Elem<'a> {
        Text::new("settings").into()
    }
}

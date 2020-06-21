use crate::gui::view::Quizers;
use iced::{Sandbox, Settings};

mod gui;
mod question;

fn main() {
    pretty_env_logger::init();
    Quizers::run(Settings::default())
}

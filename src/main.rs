use crate::gui::quizers::Quizers;
use iced::{Sandbox, Settings};

mod controls;
mod gui;
mod question;
mod question_list;

fn main() {
    pretty_env_logger::init();
    Quizers::run(Settings::default())
}

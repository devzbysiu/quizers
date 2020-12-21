use crate::quizers::Quizers;
use iced::{Sandbox, Settings};

mod controls;
mod header;
mod question;
mod question_list;
mod quizers;
mod settings;
mod settings_list;
mod style;
mod view;

fn main() {
    pretty_env_logger::init();
    Quizers::run(Settings::default()).expect("failed to run he app");
}

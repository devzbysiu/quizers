use crate::quizers::Quizers;
use iced::{Sandbox, Settings};

mod controls;
mod helpers;
mod question;
mod question_list;
mod quizers;
mod style;
mod view;

fn main() {
    pretty_env_logger::init();
    Quizers::run(Settings::default())
}

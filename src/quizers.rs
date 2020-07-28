use crate::question::Questions;
use crate::style;
use crate::view::View;
use anyhow::Result;
use iced::{Container, Element, Length, Sandbox};
use reqwest::blocking::Client;
use std::fs::read_to_string;

const QUESTIONS: &str = "res/QUESTIONS.md";

#[derive(Debug, Clone)]
pub(crate) enum Msg {
    Answer(usize),
    BackPressed,
    NextPressed,
    ShowResults,
    SettingsPressed,
    RestartPressed,
    GoBackPressed,
    GoToQuestion(usize),
}

pub(crate) struct Quizers {
    view: View,
}

impl Sandbox for Quizers {
    type Message = Msg;

    fn new() -> Self {
        let view = View::new(Questions::from(
            get_questions().expect("failed to fetch questions").as_str(),
        ));
        Self { view }
    }

    fn title(&self) -> String {
        "Quizers".into()
    }

    fn update(&mut self, event: Msg) {
        match event {
            Msg::BackPressed => self.view.go_prev_page(),
            Msg::NextPressed | Msg::ShowResults => self.view.go_next_page(),
            Msg::Answer(idx) => self.view.toggle_answer(idx),
            Msg::GoToQuestion(idx) => self.view.go_page(idx),
            Msg::SettingsPressed | Msg::GoBackPressed => self.view.go_settings_page(),
            Msg::RestartPressed => {
                // TODO: don't read questions again? - or maybe it's a feature?
                let content = read_to_string(QUESTIONS).expect("failed to read questions markdown");
                self.view = View::new(Questions::from(content.as_str()));
            }
        }
    }

    fn view(&mut self) -> Element<Msg> {
        Container::new(self.view.current())
            .height(Length::Fill)
            .width(Length::Fill)
            .style(style::Container)
            .into()
    }
}

fn get_questions() -> Result<String> {
    Ok(Client::new()
        .get("https://raw.githubusercontent.com/devzbysiu/ace-aem-sites-developer/master/QUESTIONS.md")
        .send()?
        .text()?)
}

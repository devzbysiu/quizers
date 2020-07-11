use crate::question::Questions;
use crate::style;
use crate::view::PageModel::{Question, Results, Settings};
use crate::view::View;
use iced::{Container, Element, Length, Sandbox};
use std::fs::read_to_string;

pub(crate) type Elem<'a> = Element<'a, Msg>;

const QUESTIONS: &str = "/home/zbychu/projects/md-questions/res/QUESTIONS.md";

#[derive(Debug, Clone)]
pub(crate) enum Msg {
    Answer(usize),
    BackPressed,
    NextPressed,
    ShowResults,
    SettingsPressed,
    GoBackPressed,
    GoToQuestion(usize),
}

pub(crate) struct Quizers {
    view: View,
}

impl Quizers {
    fn inner_view(&mut self) -> Elem<'_> {
        match &mut self.view.current_page {
            Question => self.view.question(),
            Results => self.view.results(),
            Settings => self.view.settings(),
        }
    }

    fn update_current_page(&mut self) {
        self.view.current_page = match self.view.page_idx {
            x if x <= self.view.questions.count() - 1 => Question,
            x if x == self.view.questions.count() => Results,
            x if x == self.view.questions.count() + 1 => Settings,
            _ => panic!("no such page"),
        }
    }
}

impl Sandbox for Quizers {
    type Message = Msg;

    fn new() -> Self {
        let content = read_to_string(QUESTIONS).expect("failed to read questions markdown");
        let view = View::new(Questions::from(content.as_str()));
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
            Msg::SettingsPressed => self.view.go_settings_page(),
            Msg::GoBackPressed => self.view.go_settings_page(),
        }
        self.update_current_page();
    }

    fn view(&mut self) -> Element<Msg> {
        Container::new(self.inner_view())
            .height(Length::Fill)
            .width(Length::Fill)
            .style(style::Container)
            .into()
    }
}

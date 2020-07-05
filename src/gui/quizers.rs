use crate::gui::style;
use crate::gui::view::{PageModel, View};
use iced::{Container, Element, Length, Sandbox};
use md_questions::Questions;
use std::fs::read_to_string;

pub(crate) type Elem<'a> = Element<'a, Msg>;

#[derive(Debug, Clone)]
pub(crate) enum Msg {
    Answer(usize),
    BackPressed,
    NextPressed,
    ShowResults,
    GoToQuestion(usize),
}

pub(crate) struct Quizers {
    view: View,
}

impl Quizers {
    fn inner_view(&'_ mut self) -> Element<'_, Msg> {
        match &mut self.view.current_page {
            PageModel::FirstQuestion => self.view.first_question(),
            PageModel::MiddleQuestion => self.view.middle_question(),
            PageModel::LastQuestion => self.view.last_question(),
            PageModel::Results => self.view.results(),
        }
    }

    fn update_current_page(&mut self) {
        self.view.current_page = match self.view.page_idx {
            0 => PageModel::FirstQuestion,
            x if x == self.view.questions.len() - 1 => PageModel::LastQuestion,
            x if x == self.view.questions.len() => PageModel::Results,
            _ => PageModel::MiddleQuestion,
        }
    }
}

impl Sandbox for Quizers {
    type Message = Msg;

    fn new() -> Self {
        let content = read_to_string("/home/zbychu/projects/md-questions/res/QUESTIONS.md")
            .expect("failed to read questions markdown");
        Self {
            view: View::new(Questions::from(content.as_str())),
        }
    }

    fn title(&self) -> String {
        "Quizers".into()
    }

    fn update(&mut self, event: Msg) {
        match event {
            Msg::BackPressed => self.view.page_idx -= 1,
            Msg::NextPressed | Msg::ShowResults => self.view.page_idx += 1,
            Msg::Answer(idx) => self.view.selected_answers[self.view.page_idx] = Some(idx),
            Msg::GoToQuestion(idx) => self.view.page_idx = idx,
        }
        self.update_current_page();
    }

    fn view(&mut self) -> Element<Msg> {
        Container::new(self.inner_view())
            .height(Length::Fill)
            .width(Length::Fill)
            .center_y()
            .style(style::Container)
            .into()
    }
}

use crate::gui::style;
use crate::gui::view::PageModel::{FirstQuestion, LastQuestion, MiddleQuestion, Results};
use crate::gui::view::{PageModel, View};
use crate::question::Questions;
use iced::{Container, Element, Length, Sandbox};
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
    fn inner_view(&mut self) -> Elem<'_> {
        match &mut self.view.current_page {
            FirstQuestion | MiddleQuestion | LastQuestion => self.view.question(),
            PageModel::Results => self.view.results(),
        }
    }

    fn update_current_page(&mut self) {
        self.view.current_page = match self.view.page_idx {
            0 => FirstQuestion,
            x if x == self.view.questions.count() - 1 => LastQuestion,
            x if x == self.view.questions.count() => Results,
            _ => MiddleQuestion,
        }
    }

    fn toggle_answer(&mut self, idx: usize) {
        self.view.questions[self.view.page_idx].toggle_answer(idx);
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
            Msg::Answer(idx) => self.toggle_answer(idx),
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

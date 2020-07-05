use crate::gui::page::{PageModel, State};
use crate::gui::{style, view};
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
    current_page: PageModel,
    state: State,
}

impl Quizers {
    fn inner_view(&'_ mut self) -> Element<'_, Msg> {
        match &mut self.current_page {
            PageModel::FirstQuestion => view::first_question(&mut self.state),
            PageModel::MiddleQuestion => view::middle_question(&mut self.state),
            PageModel::LastQuestion => view::last_question(&mut self.state),
            PageModel::Results => view::results(&mut self.state),
        }
    }

    fn update_current_page(&mut self) {
        self.current_page = match self.state.page_idx {
            0 => PageModel::FirstQuestion,
            x if x == self.state.questions.len() - 1 => PageModel::LastQuestion,
            x if x == self.state.questions.len() => PageModel::Results,
            _ => PageModel::MiddleQuestion,
        }
    }
}

impl Sandbox for Quizers {
    type Message = Msg;

    fn new() -> Self {
        let content = read_to_string("/home/zbychu/projects/md-questions/res/QUESTIONS.md")
            .expect("failed to read questions markdown");
        let questions = Questions::from(content.as_str());
        Self {
            current_page: PageModel::FirstQuestion,
            state: State::new(questions),
        }
    }

    fn title(&self) -> String {
        "Quizers".into()
    }

    fn update(&mut self, event: Msg) {
        match event {
            Msg::BackPressed => self.state.page_idx -= 1,
            Msg::NextPressed | Msg::ShowResults => self.state.page_idx += 1,
            Msg::Answer(idx) => self.state.selected_answers[self.state.page_idx] = Some(idx),
            Msg::GoToQuestion(idx) => self.state.page_idx = idx,
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

use crate::gui::page;
use crate::gui::style;
use crate::gui::view;
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
    current_page: page::PageModel,
    page_idx: usize,
    selected_answers: Vec<Option<usize>>,
    questions: Questions,
    state: page::State,
}

impl Quizers {
    fn inner_view(&'_ mut self) -> Element<'_, Msg> {
        match &mut self.current_page {
            page::PageModel::FirstQuestion => view::first_question(
                &mut self.state,
                &self.questions[self.page_idx],
                self.selected_answers[self.page_idx],
                self.page_idx,
            ),
            page::PageModel::MiddleQuestion => view::middle_question(
                &mut self.state,
                &self.questions[self.page_idx],
                self.selected_answers[self.page_idx],
                self.page_idx,
            ),
            page::PageModel::LastQuestion => view::last_question(
                &mut self.state,
                &self.questions[self.page_idx],
                self.selected_answers[self.page_idx],
                self.page_idx,
            ),
            page::PageModel::Results => view::results(
                &mut self.state,
                &self.questions,
                &self.selected_answers,
                self.page_idx,
            ),
        }
    }

    fn update_current_page(&mut self) {
        self.current_page = match self.page_idx {
            0 => page::PageModel::FirstQuestion,
            x if x == self.questions.len() - 1 => page::PageModel::LastQuestion,
            x if x == self.questions.len() => page::PageModel::Results,
            _ => page::PageModel::MiddleQuestion,
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
            current_page: page::PageModel::FirstQuestion,
            state: page::State::new(questions.len()),
            page_idx: 0,
            selected_answers: vec![None; questions.len()],
            questions,
        }
    }

    fn title(&self) -> String {
        "Quizers".into()
    }

    fn update(&mut self, event: Msg) {
        match event {
            Msg::BackPressed => self.page_idx -= 1,
            Msg::NextPressed | Msg::ShowResults => self.page_idx += 1,
            Msg::Answer(idx) => self.selected_answers[self.page_idx] = Some(idx),
            Msg::GoToQuestion(idx) => self.page_idx = idx,
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

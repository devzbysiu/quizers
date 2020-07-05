use crate::gui::page;
use crate::gui::style;
use crate::gui::view;
use iced::{scrollable, Container, Element, Length, Sandbox};
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
    scroll: scrollable::State,
}

impl Quizers {
    fn inner_view(&'_ mut self) -> Element<'_, Msg> {
        match &mut self.current_page {
            page::PageModel::FirstQuestion {
                back_button,
                next_button,
                questions_labels,
            } => view::first_question(
                back_button,
                next_button,
                questions_labels,
                &mut self.scroll,
                &self.questions[self.page_idx],
                self.selected_answers[self.page_idx],
                self.page_idx,
            ),
            page::PageModel::MiddleQuestion {
                back_button,
                next_button,
                questions_labels,
            } => view::middle_question(
                back_button,
                next_button,
                questions_labels,
                &mut self.scroll,
                &self.questions[self.page_idx],
                self.selected_answers[self.page_idx],
                self.page_idx,
            ),
            page::PageModel::LastQuestion {
                back_button,
                finish_button,
                questions_labels,
            } => view::last_question(
                back_button,
                finish_button,
                questions_labels,
                &mut self.scroll,
                &self.questions[self.page_idx],
                self.selected_answers[self.page_idx],
                self.page_idx,
            ),
            page::PageModel::Results {
                back_button,
                restart_button,
                questions_labels,
            } => view::results(
                back_button,
                restart_button,
                questions_labels,
                &mut self.scroll,
                &self.questions,
                &self.selected_answers,
                self.page_idx,
            ),
        }
    }

    fn update_current_page(&mut self) {
        self.current_page = match self.page_idx {
            0 => page::first_question(self.questions.len()),
            x if x == self.questions.len() - 1 => page::last_question(self.questions.len()),
            x if x == self.questions.len() => page::results(self.questions.len()),
            _ => page::middle_question(self.questions.len()),
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
            current_page: page::first_question(questions.len()),
            page_idx: 0,
            selected_answers: vec![None; questions.len()],
            questions,
            scroll: scrollable::State::new(),
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

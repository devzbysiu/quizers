use crate::gui::page::PageModel;
use crate::gui::style;
use crate::gui::view::{first_question, last_question, middle_question, results};
use iced::{button, scrollable, Container, Element, Length, Sandbox};
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
    page_idx: usize,
    selected_answers: Vec<Option<usize>>,
    questions: Questions,
    scroll: scrollable::State,
}

impl Quizers {
    fn inner_view(&'_ mut self) -> Element<'_, Msg> {
        match &mut self.current_page {
            PageModel::FirstQuestion {
                back_button,
                next_button,
                questions_labels,
            } => first_question(
                back_button,
                next_button,
                questions_labels,
                &mut self.scroll,
                &self.questions[self.page_idx],
                self.selected_answers[self.page_idx],
            ),
            PageModel::MiddleQuestion {
                back_button,
                next_button,
                questions_labels,
            } => middle_question(
                back_button,
                next_button,
                questions_labels,
                &mut self.scroll,
                &self.questions[self.page_idx],
                self.selected_answers[self.page_idx],
            ),
            PageModel::LastQuestion {
                back_button,
                finish_button,
                questions_labels,
            } => last_question(
                back_button,
                finish_button,
                questions_labels,
                &mut self.scroll,
                &self.questions[self.page_idx],
                self.selected_answers[self.page_idx],
            ),
            PageModel::Results {
                back_button,
                restart_button,
                questions_labels,
            } => results(
                back_button,
                restart_button,
                questions_labels,
                &mut self.scroll,
                &self.questions,
                &self.selected_answers,
            ),
        }
    }

    fn update_current_page(&mut self) {
        self.current_page = match self.page_idx {
            0 => PageModel::FirstQuestion {
                back_button: button::State::new(),
                next_button: button::State::new(),
                questions_labels: vec![button::State::new(); self.questions.len()],
            },
            x if x == self.questions.len() - 1 => PageModel::LastQuestion {
                back_button: button::State::new(),
                finish_button: button::State::new(),
                questions_labels: vec![button::State::new(); self.questions.len()],
            },
            x if x == self.questions.len() => PageModel::Results {
                back_button: button::State::new(),
                restart_button: button::State::new(),
                questions_labels: vec![button::State::new(); self.questions.len()],
            },
            _ => PageModel::MiddleQuestion {
                back_button: button::State::new(),
                next_button: button::State::new(),
                questions_labels: vec![button::State::new(); self.questions.len()],
            },
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
            current_page: PageModel::FirstQuestion {
                back_button: button::State::new(),
                next_button: button::State::new(),
                questions_labels: vec![button::State::new(); questions.len()],
            },
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
            Msg::NextPressed => self.page_idx += 1,
            Msg::Answer(idx) => self.selected_answers[self.page_idx] = Some(idx),
            Msg::ShowResults => self.page_idx += 1,
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

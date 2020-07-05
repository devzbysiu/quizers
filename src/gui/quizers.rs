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

enum PageModel {
    FirstQuestion {
        back_button: button::State,
        next_button: button::State,
        questions_labels: Vec<button::State>,
        scroll: scrollable::State,
    },
    MiddleQuestion {
        back_button: button::State,
        next_button: button::State,
        questions_labels: Vec<button::State>,
        scroll: scrollable::State,
    },
    LastQuestion {
        back_button: button::State,
        finish_button: button::State,
        questions_labels: Vec<button::State>,
        scroll: scrollable::State,
    },
    Results {
        back_button: button::State,
        restart_button: button::State,
        questions_labels: Vec<button::State>,
        scroll: scrollable::State,
    },
}

pub(crate) struct Quizers {
    current_page: PageModel,
    question_idx: usize,
    selected_answers: Vec<Option<usize>>,
    questions: Questions,
}

impl Quizers {
    fn inner_view(&'_ mut self) -> Element<'_, Msg> {
        match &mut self.current_page {
            PageModel::FirstQuestion {
                back_button,
                next_button,
                questions_labels,
                scroll,
            } => first_question(
                back_button,
                next_button,
                questions_labels,
                scroll,
                &self.questions[self.question_idx],
                self.selected_answers[self.question_idx],
            ),
            PageModel::MiddleQuestion {
                back_button,
                next_button,
                scroll,
                questions_labels,
            } => middle_question(
                back_button,
                next_button,
                questions_labels,
                scroll,
                &self.questions[self.question_idx],
                self.selected_answers[self.question_idx],
            ),
            PageModel::LastQuestion {
                back_button,
                finish_button,
                questions_labels,
                scroll,
            } => last_question(
                back_button,
                finish_button,
                questions_labels,
                scroll,
                &self.questions[self.question_idx],
                self.selected_answers[self.question_idx],
            ),
            PageModel::Results {
                back_button,
                restart_button,
                questions_labels,
                scroll,
            } => results(
                back_button,
                restart_button,
                questions_labels,
                scroll,
                &self.questions,
                &self.selected_answers,
            ),
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
                scroll: scrollable::State::new(),
                questions_labels: vec![button::State::new(); questions.len()],
            },
            question_idx: 0,
            selected_answers: vec![None; questions.len()],
            questions,
        }
    }

    fn title(&self) -> String {
        "Quizers".into()
    }

    fn update(&mut self, event: Msg) {
        match event {
            Msg::BackPressed => {
                self.question_idx -= 1;
                self.current_page = match self.question_idx {
                    x if x == 0 => PageModel::FirstQuestion {
                        back_button: button::State::new(),
                        next_button: button::State::new(),
                        questions_labels: vec![button::State::new(); self.questions.len()],
                        scroll: scrollable::State::new(),
                    },
                    _ => PageModel::MiddleQuestion {
                        back_button: button::State::new(),
                        next_button: button::State::new(),
                        questions_labels: vec![button::State::new(); self.questions.len()],
                        scroll: scrollable::State::new(),
                    },
                };
            }
            Msg::NextPressed => {
                self.question_idx += 1;
                self.current_page = match self.question_idx {
                    x if x == self.questions.len() - 1 => PageModel::LastQuestion {
                        back_button: button::State::new(),
                        finish_button: button::State::new(),
                        questions_labels: vec![button::State::new(); self.questions.len()],
                        scroll: scrollable::State::new(),
                    },
                    _ => PageModel::MiddleQuestion {
                        back_button: button::State::new(),
                        next_button: button::State::new(),
                        questions_labels: vec![button::State::new(); self.questions.len()],
                        scroll: scrollable::State::new(),
                    },
                };
            }
            Msg::Answer(idx) => {
                self.selected_answers[self.question_idx] = Some(idx);
            }
            Msg::ShowResults => {
                self.current_page = PageModel::Results {
                    back_button: button::State::new(),
                    restart_button: button::State::new(),
                    questions_labels: vec![button::State::new(); self.questions.len()],
                    scroll: scrollable::State::new(),
                }
            }
            Msg::GoToQuestion(idx) => {
                self.question_idx = idx;
                self.current_page = match self.question_idx {
                    0 => PageModel::FirstQuestion {
                        back_button: button::State::new(),
                        next_button: button::State::new(),
                        questions_labels: vec![button::State::new(); self.questions.len()],
                        scroll: scrollable::State::new(),
                    },
                    x if x == self.questions.len() - 1 => PageModel::LastQuestion {
                        back_button: button::State::new(),
                        finish_button: button::State::new(),
                        questions_labels: vec![button::State::new(); self.questions.len()],
                        scroll: scrollable::State::new(),
                    },
                    _ => PageModel::MiddleQuestion {
                        back_button: button::State::new(),
                        next_button: button::State::new(),
                        questions_labels: vec![button::State::new(); self.questions.len()],
                        scroll: scrollable::State::new(),
                    },
                };
            }
        }
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

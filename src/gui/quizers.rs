use crate::gui::style;
use crate::gui::view::{first_question, last_question, middle_question, results};
use iced::{button, Column, Container, Element, Length, Row, Sandbox, Text};
use md_questions::Questions;
use std::fs::read_to_string;

#[derive(Debug, Clone)]
pub(crate) enum Msg {
    Answer(usize),
    BackPressed,
    NextPressed,
    ShowResults,
}

enum PageModel {
    FirstQuestion {
        back_button: button::State,
        next_button: button::State,
    },
    MiddleQuestion {
        back_button: button::State,
        next_button: button::State,
    },
    LastQuestion {
        back_button: button::State,
        finish_button: button::State,
    },
    Results {
        back_button: button::State,
        restart_button: button::State,
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
        let questions_content = match &mut self.current_page {
            PageModel::FirstQuestion {
                back_button,
                next_button,
            } => first_question(
                back_button,
                next_button,
                &self.questions[self.question_idx],
                self.selected_answers[self.question_idx],
            ),
            PageModel::MiddleQuestion {
                back_button,
                next_button,
            } => middle_question(
                back_button,
                next_button,
                &self.questions[self.question_idx],
                self.selected_answers[self.question_idx],
            ),
            PageModel::LastQuestion {
                back_button,
                finish_button,
            } => last_question(
                back_button,
                finish_button,
                &self.questions[self.question_idx],
                self.selected_answers[self.question_idx],
            ),
            PageModel::Results {
                back_button,
                restart_button,
            } => results(
                back_button,
                restart_button,
                &self.questions,
                &self.selected_answers,
            ),
        };

        let column_content = Column::new()
            .push(Text::new("Question 1"))
            .push(Text::new("Question 2"))
            .push(Text::new("Question 3"))
            .push(Text::new("Question 4"));

        let questions_column = Container::new(column_content)
            .height(Length::Fill)
            .width(Length::from(150))
            .style(style::QuestionsColumn)
            .center_y();

        let questions_view = Container::new(questions_content)
            .height(Length::Fill)
            .width(Length::Fill)
            .center_x()
            .center_y();

        Row::new()
            .push(questions_column)
            .push(questions_view)
            .spacing(50)
            .into()
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
                    },
                    _ => PageModel::MiddleQuestion {
                        back_button: button::State::new(),
                        next_button: button::State::new(),
                    },
                };
            }
            Msg::NextPressed => {
                self.question_idx += 1;
                self.current_page = match self.question_idx {
                    x if x == self.questions.len() - 1 => PageModel::LastQuestion {
                        back_button: button::State::new(),
                        finish_button: button::State::new(),
                    },
                    _ => PageModel::MiddleQuestion {
                        back_button: button::State::new(),
                        next_button: button::State::new(),
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
                }
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

use crate::gui::quizers::button;
use crate::gui::style;
use iced::{
    button, scrollable, Button, Column, Container, Element, HorizontalAlignment, Length, Radio,
    Row, Sandbox, Scrollable, Space, Text,
};
use md_questions::{Question, Questions};
use std::fs::read_to_string;

#[derive(Debug, Clone)]
pub(crate) enum Msg {
    Answer(usize),
    Navigation(Page),
}

#[derive(Debug, Clone)]
pub(crate) enum Page {
    FirstQuestion,
    MiddleQuestion,
    LastQuestion,
    Results,
}

enum PageModel {
    FirstQuestion {
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
        restart_button: button::State,
    },
}

pub(crate) struct Quizers {
    current_page: PageModel,
    question_idx: usize,
    selected_answer: Option<usize>,
    questions: Questions,
}

impl Sandbox for Quizers {
    type Message = Msg;

    fn new() -> Self {
        let content = read_to_string("/home/zbychu/projects/md-questions/res/QUESTIONS.md")
            .expect("failed to read questions markdown");
        let questions = Questions::from(content.as_str());
        Self {
            current_page: PageModel::FirstQuestion {
                next_button: button::State::new(),
            },
            question_idx: 0,
            selected_answer: None,
            questions,
        }
    }

    fn title(&self) -> String {
        "Quizers".into()
    }

    fn update(&mut self, event: Msg) {
        match event {
            Msg::Navigation(page) => {
                self.current_page = match page {
                    Page::FirstQuestion => PageModel::FirstQuestion {
                        next_button: button::State::new(),
                    },
                    Page::MiddleQuestion => PageModel::MiddleQuestion {
                        back_button: button::State::new(),
                        next_button: button::State::new(),
                    },
                    Page::LastQuestion => PageModel::LastQuestion {
                        back_button: button::State::new(),
                        finish_button: button::State::new(),
                    },
                    Page::Results => PageModel::Results {
                        restart_button: button::State::new(),
                    },
                };
            }
            Msg::Answer(idx) => {}
        }
    }

    fn view(&mut self) -> Element<Msg> {
        let inner_view = match &mut self.current_page {
            PageModel::FirstQuestion { next_button } => {
                first_question_screen(next_button, &self.questions[self.question_idx])
            }
            PageModel::MiddleQuestion {
                back_button,
                next_button,
            } => {
                middle_question_screen(back_button, next_button, &self.questions[self.question_idx])
            }
            PageModel::LastQuestion {
                back_button,
                finish_button,
            } => last_question_screen(
                back_button,
                finish_button,
                &self.questions[self.question_idx],
            ),
            PageModel::Results { restart_button } => results_screen(restart_button),
        };

        Container::new(inner_view)
            .height(Length::Fill)
            .width(Length::Fill)
            .center_y()
            .center_x()
            .style(style::Container)
            .into()
    }
}

fn first_question_screen<'a>(
    next_button: &'a mut button::State,
    current_question: &'a Question,
) -> Element<'a, Msg> {
    let mut controls = Row::new();

    let button = Button::new(
        next_button,
        Text::new("Next").horizontal_alignment(HorizontalAlignment::Center),
    )
    .padding(12)
    .min_width(100)
    .style(style::Button::Primary);

    controls = controls.push(Space::with_width(Length::Fill));
    controls = controls.push(button);

    Column::new()
        .max_width(540)
        .spacing(20)
        .padding(20)
        .push(radio(current_question, None))
        .push(controls)
        .into()
}

pub(crate) fn radio<'a>(question: &Question, selected_answer: Option<usize>) -> Column<'a, Msg> {
    let q = Column::new()
        .padding(20)
        .spacing(10)
        .push((0..question.no_answers()).fold(
            Column::new().padding(10).spacing(20),
            |choices, answer| {
                choices.push(
                    Radio::new(
                        answer,
                        &question.answer(answer).text(),
                        selected_answer,
                        Msg::Answer,
                    )
                    .style(style::Radio),
                )
            },
        ));
    Column::new()
        .spacing(20)
        .push(Text::new("Question").size(50))
        .push(Text::new(&question.text()))
        .push(q)
}

fn middle_question_screen<'a>(
    back_button: &'a mut button::State,
    next_button: &'a mut button::State,
    current_question: &'a Question,
) -> Element<'a, Msg> {
    let mut controls = Row::new();

    let back_button = Button::new(
        back_button,
        Text::new("Back").horizontal_alignment(HorizontalAlignment::Center),
    )
    .padding(12)
    .min_width(100)
    .style(style::Button::Primary);

    let next_button = Button::new(
        next_button,
        Text::new("Next").horizontal_alignment(HorizontalAlignment::Center),
    )
    .padding(12)
    .min_width(100)
    .style(style::Button::Primary);

    controls = controls.push(Space::with_width(Length::Fill));
    controls = controls.push(back_button);
    controls = controls.push(next_button);

    Column::new()
        .max_width(540)
        .spacing(20)
        .padding(20)
        .push(radio(current_question, None))
        .push(controls)
        .into()
}

fn last_question_screen<'a>(
    back_button: &'a mut button::State,
    finish_button: &'a mut button::State,
    current_question: &'a Question,
) -> Element<'a, Msg> {
    let mut controls = Row::new();

    let back_button = Button::new(
        back_button,
        Text::new("Back").horizontal_alignment(HorizontalAlignment::Center),
    )
    .padding(12)
    .min_width(100)
    .style(style::Button::Primary);

    let finish_button = Button::new(
        finish_button,
        Text::new("Finish").horizontal_alignment(HorizontalAlignment::Center),
    )
    .padding(12)
    .min_width(100)
    .style(style::Button::Primary);

    controls = controls.push(Space::with_width(Length::Fill));
    controls = controls.push(back_button);
    controls = controls.push(finish_button);

    Column::new()
        .max_width(540)
        .spacing(20)
        .padding(20)
        .push(radio(current_question, None))
        .push(controls)
        .into()
}

fn results_screen<'a>(restart_button: &'a mut button::State) -> Element<'a, Msg> {
    let mut controls = Row::new();

    let restart_button = Button::new(
        restart_button,
        Text::new("Next").horizontal_alignment(HorizontalAlignment::Center),
    )
    .padding(12)
    .min_width(100)
    .style(style::Button::Primary);

    controls = controls.push(Space::with_width(Length::Fill));
    controls = controls.push(restart_button);

    Column::new()
        .max_width(540)
        .spacing(20)
        .padding(20)
        .push(controls)
        .into()
}

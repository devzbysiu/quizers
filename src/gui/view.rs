use crate::gui::helpers::{build_view, button, controls, question_view, questions_list, radio};
use crate::gui::quizers::Msg;
use conv::prelude::*;
use iced::{button, Column, Element, Text};
use md_questions::{Question, Questions};

pub(crate) fn first_question<'a>(
    back_button: &'a mut button::State,
    next_button: &'a mut button::State,
    labels: &'a mut [button::State],
    question: &'a Question,
    selected_answer: Option<usize>,
) -> Element<'a, Msg> {
    let back = button(back_button, "Back");
    let next = button(next_button, "Next").on_press(Msg::NextPressed);
    build_view(
        questions_list(labels),
        question_view(radio(question, selected_answer), controls(back, next)),
    )
}

pub(crate) fn middle_question<'a>(
    back_button: &'a mut button::State,
    next_button: &'a mut button::State,
    labels: &'a mut [button::State],
    question: &'a Question,
    selected_answer: Option<usize>,
) -> Element<'a, Msg> {
    let back = button(back_button, "Back").on_press(Msg::BackPressed);
    let next = button(next_button, "Next").on_press(Msg::NextPressed);
    build_view(
        questions_list(labels),
        question_view(radio(question, selected_answer), controls(back, next)),
    )
}

pub(crate) fn last_question<'a>(
    back_button: &'a mut button::State,
    finish_button: &'a mut button::State,
    labels: &'a mut [button::State],
    question: &'a Question,
    selected_answer: Option<usize>,
) -> Element<'a, Msg> {
    let back = button(back_button, "Back");
    let finish = button(finish_button, "Finish").on_press(Msg::ShowResults);
    build_view(
        questions_list(labels),
        question_view(radio(question, selected_answer), controls(back, finish)),
    )
}

pub(crate) fn results<'a>(
    back_button: &'a mut button::State,
    restart_button: &'a mut button::State,
    labels: &'a mut [button::State],
    questions: &Questions,
    selected_answers: &[Option<usize>],
) -> Element<'a, Msg> {
    let back = button(back_button, "Back");
    let restart = button(restart_button, "Restart");
    let mut points = 0;
    for i in 0..questions.len() {
        if let Some(idx) = selected_answers[i] {
            if questions[i].answer(idx).is_correct() {
                points += 1;
            }
        }
    }
    let result = format!(
        "You've got {}/{} ({:.2}%) points",
        points,
        questions.len(),
        f64::value_from(points).expect("failed to convert from usize to f64")
            / f64::value_from(questions.len()).expect("failed to convert from usize to f64")
    );
    let results_section = Column::new().spacing(20).push(Text::new(result));
    build_view(
        questions_list(labels),
        question_view(results_section.into(), controls(back, restart)),
    )
}

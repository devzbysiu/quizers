use crate::gui::helpers::{build_content, button, controls, radio};
use crate::gui::quizers::Msg;
use iced::{button, Column, Element, Text};
use md_questions::Question;

pub(crate) fn first_question<'a>(
    back_button: &'a mut button::State,
    next_button: &'a mut button::State,
    question: &'a Question,
) -> Element<'a, Msg> {
    let back = button(back_button, "Back");
    let next = button(next_button, "Next").on_press(Msg::NextPressed);
    build_content(radio(question, None), controls(back, next))
}

pub(crate) fn middle_question<'a>(
    back_button: &'a mut button::State,
    next_button: &'a mut button::State,
    question: &'a Question,
) -> Element<'a, Msg> {
    let back = button(back_button, "Back").on_press(Msg::BackPressed);
    let next = button(next_button, "Next").on_press(Msg::NextPressed);
    build_content(radio(question, None), controls(back, next))
}

pub(crate) fn last_question<'a>(
    back_button: &'a mut button::State,
    finish_button: &'a mut button::State,
    question: &'a Question,
) -> Element<'a, Msg> {
    let back = button(back_button, "Back");
    let finish = button(finish_button, "Finish").on_press(Msg::ShowResults);
    build_content(radio(question, None), controls(back, finish))
}

pub(crate) fn results<'a>(
    back_button: &'a mut button::State,
    restart_button: &'a mut button::State,
) -> Element<'a, Msg> {
    let back = button(back_button, "Back");
    let restart = button(restart_button, "Restart");
    let results_section = Column::new().spacing(20).push(Text::new("Results"));
    build_content(results_section.into(), controls(back, restart))
}

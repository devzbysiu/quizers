use crate::quizers::{Elem, Msg};
use crate::style;
use iced::{
    button, Button, Checkbox, Color, Column, Container, Element, HorizontalAlignment, Length,
    Radio, Row, Space, Text,
};
use std::env;

pub(crate) fn build_main_view<'a>(
    questions_list: Elem<'a>,
    header: Elem<'a>,
    questions_view: Elem<'a>,
    controls: (Button<'a, Msg>, Button<'a, Msg>),
) -> Elem<'a> {
    let question = Column::new()
        .height(Length::FillPortion(70))
        .push(questions_view);

    let controls = Row::new()
        .height(Length::FillPortion(5))
        .push(controls.0)
        .push(Space::with_width(Length::Fill))
        .push(controls.1);

    let main_view: Elem<'a> = Column::new()
        .padding(25)
        .push(header)
        .push(question)
        .push(controls)
        .into();

    debug(Row::new().push(questions_list).push(main_view).into())
}

pub(crate) fn build_settings_view<'a>(
    settings_list: Elem<'a>,
    header: Elem<'a>,
    questions_view: Elem<'a>,
) -> Elem<'a> {
    let header = Row::new()
        .height(Length::FillPortion(4))
        .push(Space::with_width(Length::Fill))
        .push(header);

    let question = Column::new()
        .height(Length::FillPortion(70))
        .push(questions_view);

    let main_view: Elem<'a> = Column::new().padding(25).push(header).push(question).into();

    debug(Row::new().push(settings_list).push(main_view).into())
}

fn debug<'a>(element: Element<'a, Msg>) -> Element<'a, Msg> {
    let mut element = element;
    match env::var("EXPLAIN_LAYOUT") {
        Ok(string) => {
            if string == "true" {
                element = element.explain(Color::BLACK);
                element
            } else {
                element
            }
        }
        _ => element,
    }
}

pub(crate) fn results_view<'a>(content: Elem<'a>) -> Elem<'a> {
    let question_with_controls = Column::new()
        .width(Length::Fill)
        .spacing(20)
        .padding(20)
        .push(content)
        .push(Space::with_height(Length::Fill));

    Container::new(question_with_controls)
        .height(Length::Fill)
        .width(Length::Fill)
        .center_x()
        .center_y()
        .into()
}

pub(crate) fn button<'a, Message>(
    state: &'a mut button::State,
    label: &str,
) -> Button<'a, Message> {
    Button::new(
        state,
        Text::new(label).horizontal_alignment(HorizontalAlignment::Center),
    )
    .padding(12)
    .min_width(150)
    .style(style::Button)
}

pub(crate) fn settings_button<'a, Message>(
    state: &'a mut button::State,
    label: &str,
) -> Button<'a, Message> {
    Button::new(
        state,
        Text::new(label).horizontal_alignment(HorizontalAlignment::Center),
    )
    .min_width(35)
    .style(style::SettingsButton)
}

pub(crate) fn radio<'a>(
    answer_idx: usize,
    answer_text: &str,
    selected_answer: Option<usize>,
) -> Elem<'a> {
    Radio::new(answer_idx, answer_text, selected_answer, Msg::Answer)
        .style(style::Radio)
        .into()
}

pub(crate) fn checkbox<'a>(is_checked: bool, answer_text: &str, answer_idx: usize) -> Elem<'a> {
    Checkbox::new(is_checked, answer_text, move |_| Msg::Answer(answer_idx))
        .size(25)
        .style(style::Checkbox)
        .into()
}

pub(crate) fn listing_label<'a, Message>(
    state: &'a mut button::State,
    label: &str,
) -> Button<'a, Message> {
    Button::new(
        state,
        Text::new(label).horizontal_alignment(HorizontalAlignment::Center),
    )
    .padding(12)
    .min_width(150)
    .style(style::QuestionLabel)
}

use crate::gui::quizers::Msg;
use crate::gui::style;
use iced::{button, Button, Column, Element, HorizontalAlignment, Length, Radio, Row, Space, Text};
use md_questions::Question;

pub(crate) fn build_content<'a>(
    content: Element<'a, Msg>,
    controls: Element<'a, Msg>,
) -> Element<'a, Msg> {
    Column::new()
        .max_width(540)
        .spacing(20)
        .padding(20)
        .push(content)
        .push(Space::with_height(Length::Fill))
        .push(controls)
        .into()
}

pub(crate) fn controls<'a>(
    left_button: Button<'a, Msg>,
    right_button: Button<'a, Msg>,
) -> Element<'a, Msg> {
    let mut controls = Row::new();
    controls = controls.push(left_button);
    controls = controls.push(Space::with_width(Length::Fill));
    controls = controls.push(right_button);
    controls.into()
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
    .min_width(100)
    .style(style::Button)
}

pub(crate) fn radio<'a>(question: &Question, selected_answer: Option<usize>) -> Element<'a, Msg> {
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
        .into()
}

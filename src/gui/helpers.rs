use crate::gui::quizers::{Elem, Msg};
use crate::gui::style;
use iced::{
    button, Button, Checkbox, Column, Container, HorizontalAlignment, Length, Radio, Row, Space,
    Text,
};

pub(crate) fn build_view<'a>(questions_list: Elem<'a>, questions_view: Elem<'a>) -> Elem<'a> {
    Row::new()
        .push(questions_list)
        .push(questions_view)
        .spacing(50)
        .into()
}

pub(crate) fn results_view<'a>(content: Elem<'a>, controls: Elem<'a>) -> Elem<'a> {
    let question_with_controls = Column::new()
        .max_width(1366)
        .width(Length::Fill)
        .spacing(20)
        .padding(20)
        .push(content)
        .push(Space::with_height(Length::Fill))
        .push(controls);

    Container::new(question_with_controls)
        .height(Length::Fill)
        .width(Length::Fill)
        .center_x()
        .center_y()
        .into()
}

pub(crate) fn controls<'a>(
    left_button: Button<'a, Msg>,
    right_button: Button<'a, Msg>,
) -> Elem<'a> {
    Row::new()
        .push(left_button)
        .push(Space::with_width(Length::Fill))
        .push(right_button)
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

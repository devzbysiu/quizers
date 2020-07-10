use crate::gui::quizers::{Elem, Msg};
use crate::gui::style;
use iced::{
    button, Button, Checkbox, Column, Container, HorizontalAlignment, Length, Radio, Row, Space,
    Text,
};

pub(crate) fn build_view<'a>(
    questions_list: Elem<'a>,
    questions_view: Elem<'a>,
    controls: (Button<'a, Msg>, Button<'a, Msg>),
) -> Elem<'a> {
    let header = Column::new()
        .max_width(1366)
        .height(Length::FillPortion(5))
        .spacing(20)
        .padding(20)
        .push(Text::new("header"));

    let question = Column::new()
        .max_width(1366)
        .height(Length::FillPortion(70))
        .spacing(20)
        .padding(20)
        .push(questions_view);

    let controls_row = Row::new()
        .push(controls.0)
        .push(Space::with_width(Length::Fill))
        .push(controls.1);

    let controls = Column::new()
        .max_width(1366)
        .height(Length::FillPortion(8))
        .spacing(20)
        .padding(20)
        .push(controls_row);

    let main_view: Elem<'a> = Column::new()
        .push(header)
        .push(question)
        .push(controls)
        .into();

    Row::new()
        .push(questions_list)
        .push(main_view)
        .spacing(50)
        .into()
}

pub(crate) fn results_view<'a>(content: Elem<'a>) -> Elem<'a> {
    let question_with_controls = Column::new()
        .max_width(1366)
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

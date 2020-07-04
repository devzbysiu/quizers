use crate::gui::quizers::Msg;
use crate::gui::style;
use iced::{
    button, Button, Column, Container, Element, HorizontalAlignment, Length, Radio, Row, Space,
    Text,
};
use md_questions::Question;

pub(crate) fn build_content<'a>(
    questions_labels: Element<'a, Msg>,
    content: Element<'a, Msg>,
    controls: Element<'a, Msg>,
) -> Element<'a, Msg> {
    let questions_content = Column::new()
        .max_width(540)
        .spacing(20)
        .padding(20)
        .push(content)
        .push(Space::with_height(Length::Fill))
        .push(controls);

    let questions_view = Container::new(questions_content)
        .height(Length::Fill)
        .width(Length::Fill)
        .center_x()
        .center_y();

    Row::new()
        .push(questions_labels)
        .push(questions_view)
        .spacing(50)
        .into()
}

pub(crate) fn questions_labels<'a>(questions_labels: &'a mut [button::State]) -> Element<'a, Msg> {
    let mut column_content = Column::new();
    for (idx, question) in questions_labels.iter_mut().enumerate() {
        column_content = column_content.push(
            question_label(question, &format!("Question {}", idx + 1))
                .on_press(Msg::GoToQuestion(idx)),
        );
        column_content = column_content.push(Space::with_height(Length::from(10)));
    }

    Container::new(column_content)
        .height(Length::Fill)
        .width(Length::from(150))
        .style(style::QuestionsColumn)
        .center_y()
        .into()
}

pub(crate) fn question_label<'a, Message>(
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

pub(crate) fn controls<'a>(
    left_button: Button<'a, Msg>,
    right_button: Button<'a, Msg>,
) -> Element<'a, Msg> {
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

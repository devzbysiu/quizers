use crate::gui::quizers::{Elem, Msg};
use crate::gui::style;
use iced::{
    button, scrollable, Button, Column, Container, HorizontalAlignment, Length, Radio, Row,
    Scrollable, Space, Text,
};
use md_questions::{Answer, Question};

pub(crate) fn build_view<'a>(questions_list: Elem<'a>, questions_view: Elem<'a>) -> Elem<'a> {
    Row::new()
        .push(questions_list)
        .push(questions_view)
        .spacing(50)
        .into()
}

pub(crate) fn question_view<'a>(content: Elem<'a>, controls: Elem<'a>) -> Elem<'a> {
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

pub(crate) fn questions_list<'a>(
    scroll: &'a mut scrollable::State,
    questions_labels: &'a mut [button::State],
) -> Elem<'a> {
    let mut column_content = Column::new();
    for (idx, question) in questions_labels.iter_mut().enumerate() {
        column_content = column_content.push(
            question_label(question, &format!("Question {}", idx + 1))
                .on_press(Msg::GoToQuestion(idx)),
        );
        column_content = column_content.push(Space::with_height(Length::from(10)));
    }

    let scrollable_column = Scrollable::new(scroll).push(
        Container::new(column_content)
            .width(Length::Fill)
            .center_x(),
    );

    Container::new(scrollable_column)
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

pub(crate) fn question_text<'a>(question: &Question, selected_answer: Option<usize>) -> Elem<'a> {
    Column::new()
        .spacing(20)
        .push(Text::new("Question").size(50))
        .push(Text::new(&question.text()))
        .push(answers(question.answers(), selected_answer))
        .into()
}

fn answers<'a>(answers: &[Answer], selected_answer: Option<usize>) -> Elem<'a> {
    Column::new()
        .padding(20)
        .spacing(10)
        .push(
            (0..answers.len()).fold(Column::new().spacing(20), |choices, answer_idx| {
                choices.push(radio(
                    answer_idx,
                    &answers[answer_idx].text(),
                    selected_answer,
                ))
            }),
        )
        .into()
}

fn radio<'a>(answer_idx: usize, answer_text: &str, selected_answer: Option<usize>) -> Elem<'a> {
    Radio::new(answer_idx, answer_text, selected_answer, Msg::Answer)
        .style(style::Radio)
        .into()
}

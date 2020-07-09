use crate::gui::quizers::{Elem, Msg};
use crate::gui::style;
use iced::{
    button, scrollable, Button, Column, Container, HorizontalAlignment, Length, Scrollable, Space,
    Text,
};

pub(crate) struct QuestionList {
    pub(crate) questions_labels: Vec<button::State>,
    pub(crate) scroll: scrollable::State,
}

impl QuestionList {
    pub(crate) fn new(question_count: usize) -> Self {
        Self {
            questions_labels: vec![button::State::new(); question_count],
            scroll: scrollable::State::new(),
        }
    }

    pub(crate) fn view(&mut self, selected_question: usize) -> Elem<'_> {
        let mut column_content = Column::new();
        for (idx, question) in self.questions_labels.iter_mut().enumerate() {
            let mut label = question_label(question, &format!("Question {}", idx + 1))
                .on_press(Msg::GoToQuestion(idx));
            if selected_question == idx {
                label = label.style(style::SelectedLabel);
            }
            column_content = column_content.push(label);
            column_content = column_content.push(Space::with_height(Length::from(10)));
        }

        let scrollable_column = Scrollable::new(&mut self.scroll).push(
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
}

fn question_label<'a, Message>(state: &'a mut button::State, label: &str) -> Button<'a, Message> {
    Button::new(
        state,
        Text::new(label).horizontal_alignment(HorizontalAlignment::Center),
    )
    .padding(12)
    .min_width(150)
    .style(style::QuestionLabel)
}

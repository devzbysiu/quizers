use crate::quizers::Msg;
use crate::style;
use crate::view::listing_label;
use crate::view::Elem;
use iced::{button, scrollable, Column, Container, Length, Scrollable};

pub(crate) struct QuestionList {
    questions_labels: Vec<button::State>,
    scroll: scrollable::State,
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
            let mut label = listing_label(question, &format!("Question {}", idx + 1))
                .on_press(Msg::GoToQuestion(idx));
            if selected_question == idx {
                label = label.style(style::SelectedLabel);
            }
            column_content = column_content.push(label);
        }

        Scrollable::new(&mut self.scroll)
            .push(
                Container::new(column_content)
                    .width(Length::Fill)
                    .center_x(),
            )
            .into()
    }

    pub(crate) fn mark_questions(&mut self, questions_state: &[bool]) -> Elem<'_> {
        let mut column_content = Column::new();
        for (idx, question) in self.questions_labels.iter_mut().enumerate() {
            let mut label = listing_label(question, &format!("Question {}", idx + 1))
                .on_press(Msg::GoToQuestion(idx));
            if questions_state[idx] {
                label = label.style(style::CorrectAnswer);
            } else {
                label = label.style(style::WrongAnswer);
            }
            column_content = column_content.push(label);
        }

        Scrollable::new(&mut self.scroll)
            .push(
                Container::new(column_content)
                    .width(Length::Fill)
                    .center_x(),
            )
            .into()
    }
}

use crate::gui::style;
use crate::question::QuestionsProvider;
use iced::{Column, Element, Radio, Text};
use md_questions::Question;

pub(crate) struct QuestionsView {
    questions_provider: Box<dyn QuestionsProvider>,
    current: usize,
    selected_answer: usize,
}

impl QuestionsView {
    pub(crate) fn new(questions_provider: Box<dyn QuestionsProvider>) -> QuestionsView {
        QuestionsView {
            questions_provider,
            current: 0,
            selected_answer: 0,
        }
    }

    pub(crate) fn update(&mut self, msg: &QuestionMessage) {
        self.update_view(msg);
    }

    fn update_view(&mut self, msg: &QuestionMessage) {
        match msg {
            QuestionMessage::Answered(selected) => {
                self.selected_answer = *selected;
            }
        };
    }

    fn container<'a>(title: &str) -> Column<'a, QuestionMessage> {
        Column::new().spacing(20).push(Text::new(title).size(50))
    }

    pub(crate) fn view(&mut self) -> Element<QuestionMessage> {
        Self::radio(
            &self.questions_provider.question(self.current),
            self.selected_answer,
        )
        .into()
    }

    fn radio<'a>(question: &Question, selected_answer: usize) -> Column<'a, QuestionMessage> {
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
                            Some(selected_answer),
                            QuestionMessage::Answered,
                        )
                        .style(style::Radio),
                    )
                },
            ));

        Self::container("Question")
            .push(Text::new(&question.text()))
            .push(q)
    }

    pub(crate) fn advance(&mut self) {
        if self.can_continue() {
            self.current += 1;
        }
    }

    pub(crate) fn go_back(&mut self) {
        if self.has_previous() {
            self.current -= 1;
        }
    }

    pub(crate) fn has_previous(&self) -> bool {
        self.current > 0
    }

    pub(crate) fn can_continue(&self) -> bool {
        self.current + 1 < self.questions_provider.len()
    }
}

#[derive(Debug, Clone)]
pub enum QuestionMessage {
    Answered(usize),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Answers {
    First,
    Second,
    Third,
    Fourth,
    Fifth,
}

impl From<Answers> for usize {
    fn from(answer: Answers) -> usize {
        match answer {
            Answers::First => 0,
            Answers::Second => 1,
            Answers::Third => 2,
            Answers::Fourth => 3,
            Answers::Fifth => 4,
        }
    }
}

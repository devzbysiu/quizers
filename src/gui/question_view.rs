use crate::gui::{style, State};
use crate::question::QuestionsProvider;
use iced::{Column, Element, Radio, Text};
use md_questions::Question;

pub(crate) struct QuestionsView {
    questions_provider: Box<dyn QuestionsProvider>,
}

impl QuestionsView {
    pub(crate) fn new(questions_provider: Box<dyn QuestionsProvider>) -> QuestionsView {
        QuestionsView { questions_provider }
    }

    pub(crate) fn view(&self, state: &State) -> Element<QuestionMessage> {
        if state.show_results {
            Self::container("Results")
                .push(Text::new("You've got 75%"))
                .into()
        } else {
            Self::radio(
                &self.questions_provider.question(state.current),
                state.selected_answer,
            )
            .into()
        }
    }

    fn radio<'a>(
        question: &Question,
        selected_answer: Option<usize>,
    ) -> Column<'a, QuestionMessage> {
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

    fn container<'a>(title: &str) -> Column<'a, QuestionMessage> {
        Column::new().spacing(20).push(Text::new(title).size(50))
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

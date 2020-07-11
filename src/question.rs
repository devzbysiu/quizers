use crate::helpers::{checkbox, radio};
use crate::quizers::Elem;
use iced::{Column, Container, Length, Text};
use md_questions::{MdQuestion, MdQuestions};
use std::mem;
use std::ops::{Index, IndexMut};

pub(crate) struct Questions {
    questions: Vec<Question>,
}

impl Questions {
    pub(crate) fn new(md_questions: &MdQuestions) -> Self {
        let mut questions = vec![];
        let mut md_questions = md_questions.questions().to_vec();
        for (idx, question) in md_questions.iter_mut().enumerate() {
            questions.push(Question::new(mem::take(question), idx));
        }
        Self { questions }
    }

    pub(crate) fn count(&self) -> usize {
        self.questions.len()
    }

    pub(crate) fn count_points(&self) -> u32 {
        let mut points = 0;
        for i in 0..self.questions.len() {
            points += self.questions[i].points_gained();
        }
        points
    }
}

impl IndexMut<usize> for Questions {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.questions.get_mut(index).unwrap()
    }
}

impl Index<usize> for Questions {
    type Output = Question;
    fn index(&self, index: usize) -> &Self::Output {
        &self.questions[index]
    }
}

impl From<&str> for Questions {
    fn from(content: &str) -> Self {
        Questions::new(&MdQuestions::from(content))
    }
}

pub(crate) struct Question {
    idx: usize,
    md_question: MdQuestion,
    selected_answers: Vec<bool>,
}

impl Question {
    fn new(md_question: MdQuestion, idx: usize) -> Self {
        Self {
            idx,
            selected_answers: vec![false; md_question.answers_count()],
            md_question,
        }
    }

    pub(crate) fn toggle_answer(&mut self, idx: usize) {
        if self.md_question.is_multi() {
            self.selected_answers[idx] = !self.selected_answers[idx];
        } else {
            self.selected_answers = vec![false; self.md_question.answers_count()];
            self.selected_answers[idx] = true;
        }
    }

    pub(crate) fn points_gained(&self) -> u32 {
        let mut correct_answers = 0;
        for j in 0..self.answers_count() {
            if self.selected_answers[j] {
                if self.is_answer_correct(j) {
                    correct_answers += 1;
                } else {
                    correct_answers = 0;
                }
            }
        }
        if correct_answers == self.correct_answers_count() {
            1
        } else {
            0
        }
    }

    pub(crate) fn answers_count(&self) -> usize {
        self.md_question.answers_count()
    }

    fn is_answer_correct(&self, idx: usize) -> bool {
        self.md_question.answer(idx).is_correct()
    }

    pub(crate) fn correct_answers_count(&self) -> usize {
        self.md_question
            .answers()
            .iter()
            .filter(|answer| answer.is_correct())
            .count()
    }

    pub(crate) fn answer_text(&self, idx: usize) -> String {
        self.md_question.answer(idx).text()
    }

    pub(crate) fn is_multi(&self) -> bool {
        self.md_question.is_multi()
    }

    pub(crate) fn text(&self) -> String {
        self.md_question.text()
    }

    pub(crate) fn view<'a>(&self) -> Elem<'a> {
        Container::new(self.question_text())
            .height(Length::Fill)
            .width(Length::Fill)
            .center_x()
            .into()
    }

    fn question_text<'a>(&self) -> Elem<'a> {
        Column::new()
            .spacing(20)
            .push(Text::new(format!("Question {}", self.idx + 1)).size(50))
            .push(Text::new(self.text()))
            .push(self.answers())
            .into()
    }

    fn answers<'a>(&self) -> Elem<'a> {
        if self.is_multi() {
            Column::new()
                .padding(20)
                .spacing(10)
                .push((0..self.answers_count()).fold(
                    Column::new().spacing(20),
                    |choices, answer_idx| {
                        choices.push(checkbox(
                            self.selected_answers[answer_idx],
                            &self.answer_text(answer_idx),
                            answer_idx,
                        ))
                    },
                ))
                .into()
        } else {
            Column::new()
                .padding(20)
                .spacing(10)
                .push((0..self.answers_count()).fold(
                    Column::new().spacing(20),
                    |choices, answer_idx| {
                        let selected_answer = if self.selected_answers[answer_idx] {
                            Some(answer_idx)
                        } else {
                            None
                        };
                        choices.push(radio(
                            answer_idx,
                            &self.answer_text(answer_idx),
                            selected_answer,
                        ))
                    },
                ))
                .into()
        }
    }
}

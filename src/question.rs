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
        for question in &mut md_questions {
            questions.push(Question::new(mem::take(question)));
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
    md_question: MdQuestion,
    selected_answers: Vec<bool>,
}

impl Question {
    fn new(md_question: MdQuestion) -> Self {
        Self {
            selected_answers: vec![false; md_question.answers_count()],
            md_question,
        }
    }

    pub(crate) fn selected_answers(&self) -> &[bool] {
        &self.selected_answers
    }

    pub(crate) fn toggle_answer(&mut self, idx: usize) {
        self.selected_answers[idx] = !self.selected_answers[idx];
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
}

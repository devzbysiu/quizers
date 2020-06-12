use md_questions::{Question, Questions};

pub(crate) trait QuestionsProvider {
    fn questions(&self) -> &[Question];
    fn question(&self, idx: usize) -> &Question;
    fn len(&self) -> usize;
}

impl QuestionsProvider for Questions {
    fn questions(&self) -> &[Question] {
        self.questions()
    }

    fn question(&self, idx: usize) -> &Question {
        self.question(idx)
    }

    fn len(&self) -> usize {
        self.len()
    }
}

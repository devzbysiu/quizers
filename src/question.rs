use md_questions::{Question, Questions};

// FIXME: this shouldn't use structured from md_questions
//  either use separate abstraction or make those
//  structures common between md_questions and this app
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

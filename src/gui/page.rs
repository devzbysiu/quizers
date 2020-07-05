use crate::gui::helpers::{
    build_view, button, controls, question_text, question_view, questions_list,
};
use crate::gui::quizers::{Elem, Msg};
use conv::prelude::*;
use iced::{button, scrollable, Column, Text};
use md_questions::Questions;

pub(crate) enum PageModel {
    FirstQuestion,
    MiddleQuestion,
    LastQuestion,
    Results,
}

pub(crate) struct View {
    pub(crate) back_button: button::State,
    pub(crate) next_button: button::State,
    pub(crate) finish_button: button::State,
    pub(crate) restart_button: button::State,
    pub(crate) questions_labels: Vec<button::State>,
    pub(crate) scroll: scrollable::State,
    pub(crate) selected_answers: Vec<Option<usize>>,
    pub(crate) questions: Questions,
    pub(crate) page_idx: usize,
    pub(crate) current_page: PageModel,
}

impl View {
    pub(crate) fn new(questions: Questions) -> Self {
        Self {
            back_button: button::State::new(),
            next_button: button::State::new(),
            finish_button: button::State::new(),
            restart_button: button::State::new(),
            questions_labels: vec![button::State::new(); questions.len()],
            scroll: scrollable::State::new(),
            selected_answers: vec![None; questions.len()],
            questions,
            page_idx: 0,
            current_page: PageModel::FirstQuestion,
        }
    }

    pub(crate) fn first_question<'a>(&'a mut self) -> Elem<'a> {
        let back = button(&mut self.back_button, "Back");
        let next = button(&mut self.next_button, "Next").on_press(Msg::NextPressed);
        build_view(
            questions_list(&mut self.scroll, &mut self.questions_labels, self.page_idx),
            question_view(
                question_text(
                    &self.questions[self.page_idx],
                    self.selected_answers[self.page_idx],
                ),
                controls(back, next),
            ),
        )
    }

    pub(crate) fn middle_question<'a>(&'a mut self) -> Elem<'a> {
        let back = button(&mut self.back_button, "Back").on_press(Msg::BackPressed);
        let next = button(&mut self.next_button, "Next").on_press(Msg::NextPressed);
        build_view(
            questions_list(&mut self.scroll, &mut self.questions_labels, self.page_idx),
            question_view(
                question_text(
                    &self.questions[self.page_idx],
                    self.selected_answers[self.page_idx],
                ),
                controls(back, next),
            ),
        )
    }

    pub(crate) fn last_question<'a>(&'a mut self) -> Elem<'a> {
        let back = button(&mut self.back_button, "Back");
        let finish = button(&mut self.finish_button, "Finish").on_press(Msg::ShowResults);
        build_view(
            questions_list(&mut self.scroll, &mut self.questions_labels, self.page_idx),
            question_view(
                question_text(
                    &self.questions[self.page_idx],
                    self.selected_answers[self.page_idx],
                ),
                controls(back, finish),
            ),
        )
    }

    pub(crate) fn results<'a>(&'a mut self) -> Elem<'a> {
        let back = button(&mut self.back_button, "Back");
        let restart = button(&mut self.restart_button, "Restart");
        let points = count_points(&self.questions, &self.selected_answers);
        let result = format_result_msg(points, self.questions.len());
        let results_section = Column::new().spacing(20).push(Text::new(result));
        build_view(
            questions_list(&mut self.scroll, &mut self.questions_labels, self.page_idx),
            question_view(results_section.into(), controls(back, restart)),
        )
    }
}

fn count_points(questions: &Questions, selected_answers: &[Option<usize>]) -> u32 {
    let mut points = 0;
    for i in 0..questions.len() {
        if let Some(idx) = selected_answers[i] {
            if questions[i].answer(idx).is_correct() {
                points += 1;
            }
        }
    }
    points
}

fn format_result_msg(points: u32, questions_count: usize) -> String {
    format!(
        "You've got {}/{} ({:.2}%) points",
        points,
        questions_count,
        f64::value_from(points).expect("failed to convert from usize to f64")
            / f64::value_from(questions_count).expect("failed to convert from usize to f64")
    )
}

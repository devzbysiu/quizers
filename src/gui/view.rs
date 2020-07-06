use crate::gui::helpers::{
    build_view, button, controls, question_text, question_view, questions_list,
};
use crate::gui::quizers::{Elem, Msg};
use crate::question::Questions;
use conv::prelude::*;
use iced::{button, scrollable, Column, Text};

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
            questions_labels: vec![button::State::new(); questions.count()],
            scroll: scrollable::State::new(),
            questions,
            page_idx: 0,
            current_page: PageModel::FirstQuestion,
        }
    }

    pub(crate) fn first_question(&mut self) -> Elem<'_> {
        let back = button(&mut self.back_button, "Back");
        let next = button(&mut self.next_button, "Next").on_press(Msg::NextPressed);
        build_view(
            questions_list(&mut self.scroll, &mut self.questions_labels, self.page_idx),
            question_view(
                question_text(&self.questions[self.page_idx], self.page_idx),
                controls(back, next),
            ),
        )
    }

    pub(crate) fn middle_question(&mut self) -> Elem<'_> {
        let back = button(&mut self.back_button, "Back").on_press(Msg::BackPressed);
        let next = button(&mut self.next_button, "Next").on_press(Msg::NextPressed);
        build_view(
            questions_list(&mut self.scroll, &mut self.questions_labels, self.page_idx),
            question_view(
                question_text(&self.questions[self.page_idx], self.page_idx),
                controls(back, next),
            ),
        )
    }

    pub(crate) fn last_question(&mut self) -> Elem<'_> {
        let back = button(&mut self.back_button, "Back");
        let finish = button(&mut self.finish_button, "Finish").on_press(Msg::ShowResults);
        build_view(
            questions_list(&mut self.scroll, &mut self.questions_labels, self.page_idx),
            question_view(
                question_text(&self.questions[self.page_idx], self.page_idx),
                controls(back, finish),
            ),
        )
    }

    pub(crate) fn results(&mut self) -> Elem<'_> {
        let back = button(&mut self.back_button, "Back");
        let restart = button(&mut self.restart_button, "Restart");
        let result = format_result_msg(&self.questions);
        let results_section = Column::new().spacing(20).push(Text::new(result));
        build_view(
            questions_list(&mut self.scroll, &mut self.questions_labels, self.page_idx),
            question_view(results_section.into(), controls(back, restart)),
        )
    }
}

fn format_result_msg(questions: &Questions) -> String {
    let points = questions.count_points();
    let questions_count = questions.count();
    format!(
        "You've got {}/{} ({:.2}%) points",
        points,
        questions.count(),
        f64::value_from(points).expect("failed to convert from usize to f64")
            / f64::value_from(questions_count).expect("failed to convert from usize to f64")
    )
}

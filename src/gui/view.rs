use crate::gui::helpers::{build_view, button, results_view};
use crate::gui::quizers::{Elem, Msg};
use crate::question::Questions;
use crate::question_list::QuestionList;
use conv::prelude::*;
use iced::{button, Button, Column, Text};

pub(crate) enum PageModel {
    FirstQuestion,
    MiddleQuestion,
    LastQuestion,
    Settings,
    Results,
}

pub(crate) struct View {
    pub(crate) back_button: button::State,
    pub(crate) next_button: button::State,
    pub(crate) finish_button: button::State,
    pub(crate) restart_button: button::State,
    pub(crate) questions_list: QuestionList,
    pub(crate) questions: Questions,
    pub(crate) page_idx: usize,
    pub(crate) questions_count: usize,
    pub(crate) current_page: PageModel,
}

impl View {
    pub(crate) fn new(questions: Questions) -> Self {
        Self {
            back_button: button::State::new(),
            next_button: button::State::new(),
            finish_button: button::State::new(),
            restart_button: button::State::new(),
            questions_list: QuestionList::new(questions.count()),
            questions_count: questions.count(),
            questions,
            page_idx: 0,
            current_page: PageModel::FirstQuestion,
        }
    }

    pub(crate) fn question(&mut self) -> Elem<'_> {
        build_view(
            self.questions_list.view(self.page_idx),
            self.questions[self.page_idx].view(),
            ctrls(
                self.page_idx,
                self.questions_count,
                &mut self.back_button,
                &mut self.next_button,
                &mut self.finish_button,
                &mut self.restart_button,
            ),
        )
    }

    pub(crate) fn results(&mut self) -> Elem<'_> {
        let result = format_result_msg(&self.questions);
        let results_section = Column::new().spacing(20).push(Text::new(result));
        build_view(
            self.questions_list.view(self.page_idx),
            results_view(results_section.into()),
            ctrls(
                self.page_idx,
                self.questions_count,
                &mut self.back_button,
                &mut self.next_button,
                &mut self.finish_button,
                &mut self.restart_button,
            ),
        )
    }

    pub(crate) fn settings<'a>(&mut self) -> Elem<'a> {
        let result = format_result_msg(&self.questions);
        let results_section = Column::new().spacing(20).push(Text::new(result));
        results_section.into()
    }
}

fn ctrls<'a>(
    page_idx: usize,
    questions_count: usize,
    back_button: &'a mut button::State,
    next_button: &'a mut button::State,
    finish_button: &'a mut button::State,
    restart_button: &'a mut button::State,
) -> (Button<'a, Msg>, Button<'a, Msg>) {
    match page_idx {
        0 => {
            let back = button(back_button, "Back");
            let next = button(next_button, "Next").on_press(Msg::NextPressed);
            (back.into(), next.into())
        }
        x if x == questions_count - 1 => {
            let back = button(back_button, "Back");
            let finish = button(finish_button, "Finish").on_press(Msg::ShowResults);
            (back.into(), finish.into())
        }
        x if x == questions_count => {
            let back = button(back_button, "Back");
            let restart = button(restart_button, "Restart");
            (back.into(), restart.into())
        }
        _ => {
            let back = button(back_button, "Back").on_press(Msg::BackPressed);
            let next = button(next_button, "Next").on_press(Msg::NextPressed);
            (back.into(), next.into())
        }
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

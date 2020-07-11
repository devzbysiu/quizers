use crate::controls::Controls;
use crate::gui::helpers::{build_view, results_view};
use crate::gui::quizers::Elem;
use crate::question::Questions;
use crate::question_list::QuestionList;
use conv::prelude::*;
use iced::{Column, Text};

pub(crate) enum PageModel {
    FirstQuestion,
    MiddleQuestion,
    LastQuestion,
    Settings,
    Results,
}

pub(crate) struct View {
    pub(crate) controls: Controls,
    pub(crate) questions_list: QuestionList,
    pub(crate) questions: Questions,
    pub(crate) page_idx: usize,
    pub(crate) current_page: PageModel,
}

impl View {
    pub(crate) fn new(questions: Questions) -> Self {
        Self {
            controls: Controls::new(questions.count()),
            questions_list: QuestionList::new(questions.count()),
            questions,
            page_idx: 0,
            current_page: PageModel::FirstQuestion,
        }
    }

    pub(crate) fn question(&mut self) -> Elem<'_> {
        build_view(
            self.questions_list.view(self.page_idx),
            self.questions[self.page_idx].view(),
            self.controls.ctrls(self.page_idx),
        )
    }

    pub(crate) fn results(&mut self) -> Elem<'_> {
        let result = format_result_msg(&self.questions);
        let results_section = Column::new().spacing(20).push(Text::new(result));
        build_view(
            self.questions_list.view(self.page_idx),
            results_view(results_section.into()),
            self.controls.ctrls(self.page_idx),
        )
    }

    pub(crate) fn settings<'a>(&mut self) -> Elem<'a> {
        let result = format_result_msg(&self.questions);
        let results_section = Column::new().spacing(20).push(Text::new(result));
        results_section.into()
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

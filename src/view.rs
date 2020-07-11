use crate::controls::Controls;
use crate::header::Header;
use crate::helpers::{build_view, results_view};
use crate::question::Questions;
use crate::question_list::QuestionList;
use crate::quizers::Elem;
use conv::prelude::*;
use iced::{Column, Text};

pub(crate) enum PageModel {
    Question,
    Settings,
    Results,
}

pub(crate) struct View {
    header: Header,
    controls: Controls,
    questions_list: QuestionList,
    pub(crate) questions: Questions,
    pub(crate) page_idx: usize,
    pub(crate) current_page: PageModel,
}

impl View {
    pub(crate) fn new(questions: Questions) -> Self {
        Self {
            header: Header::new(),
            controls: Controls::new(questions.count()),
            questions_list: QuestionList::new(questions.count()),
            questions,
            page_idx: 0,
            current_page: PageModel::Question,
        }
    }

    pub(crate) fn question(&mut self) -> Elem<'_> {
        build_view(
            self.questions_list.view(self.page_idx),
            self.header.view(),
            self.questions[self.page_idx].view(),
            self.controls.ctrls(self.page_idx),
        )
    }

    pub(crate) fn results(&mut self) -> Elem<'_> {
        let result = format_result_msg(&self.questions);
        let results_section = Column::new().spacing(20).push(Text::new(result));
        build_view(
            self.questions_list.view(self.page_idx),
            self.header.view(),
            results_view(results_section.into()),
            self.controls.ctrls(self.page_idx),
        )
    }

    pub(crate) fn settings<'a>(&mut self) -> Elem<'a> {
        Column::new()
            .spacing(20)
            .push(Text::new("settings section"))
            .into()
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

use crate::controls::Controls;
use crate::header::Header;
use crate::helpers::{build_main_view, build_settings_view, results_view};
use crate::question::Questions;
use crate::question_list::QuestionList;
use crate::quizers::Elem;
use crate::settings::Settings;
use crate::settings_list::SettingsList;
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
    settings_list: SettingsList,
    settings: Settings,
    pub(crate) questions: Questions,
    pub(crate) page_idx: usize,
    setting_idx: usize,
    pub(crate) current_page: PageModel,
}

impl View {
    pub(crate) fn new(questions: Questions) -> Self {
        Self {
            header: Header::new(),
            controls: Controls::new(questions.count()),
            questions_list: QuestionList::new(questions.count()),
            settings_list: SettingsList::new(),
            settings: Settings::new(),
            questions,
            page_idx: 0,
            setting_idx: 0,
            current_page: PageModel::Question,
        }
    }

    pub(crate) fn question(&mut self) -> Elem<'_> {
        build_main_view(
            self.questions_list.view(self.page_idx),
            self.header.view(),
            self.questions[self.page_idx].view(),
            self.controls.ctrls(self.page_idx),
        )
    }

    pub(crate) fn results(&mut self) -> Elem<'_> {
        let result = format_result_msg(&self.questions);
        let results_section = Column::new().spacing(20).push(Text::new(result));
        build_main_view(
            self.questions_list.view(self.page_idx),
            self.header.view(),
            results_view(results_section.into()),
            self.controls.ctrls(self.page_idx),
        )
    }

    pub(crate) fn settings(&mut self) -> Elem<'_> {
        build_settings_view(
            self.settings_list.view(self.setting_idx),
            self.header.view(),
            self.settings.view(),
        )
    }

    pub(crate) fn go_next_page(&mut self) {
        self.page_idx += 1;
    }

    pub(crate) fn go_prev_page(&mut self) {
        self.page_idx -= 1;
    }

    pub(crate) fn go_page(&mut self, page_idx: usize) {
        self.page_idx = page_idx;
    }

    pub(crate) fn toggle_answer(&mut self, idx: usize) {
        self.questions[self.page_idx].toggle_answer(idx);
    }

    pub(crate) fn go_settings_page(&mut self) {
        self.page_idx = self.questions.count() + 1;
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

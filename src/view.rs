use crate::controls::Controls;
use crate::header::Header;
use crate::question::Questions;
use crate::question_list::QuestionList;
use crate::quizers::Msg;
use crate::settings::Settings;
use crate::settings_list::SettingsList;
use crate::style;
use conv::prelude::*;
use iced::{
    button, Button, Checkbox, Color, Column, Container, Element, HorizontalAlignment, Length,
    Radio, Row, Space, Text,
};
use std::env;

pub(crate) type Elem<'a> = Element<'a, Msg>;
pub(crate) type Buttons<'a> = (Button<'a, Msg>, Button<'a, Msg>);

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

pub(crate) fn build_main_view<'a>(
    questions_list: Elem<'a>,
    header: Elem<'a>,
    question_view: Elem<'a>,
    buttons: Buttons<'a>,
) -> Elem<'a> {
    debug(
        Column::new()
            .push(control_bar_container(header))
            .push(quiz_view_row(
                questions_list_container(questions_list),
                question_with_controls_column(question_view, controls_row(buttons)),
            ))
            .into(),
    )
}

fn control_bar_container(header: Elem<'_>) -> Elem<'_> {
    Container::new(header)
        .height(Length::Units(50))
        .style(style::Header)
        .into()
}

fn quiz_view_row<'a>(
    questions_list_container: Elem<'a>,
    question_with_controls: Elem<'a>,
) -> Elem<'a> {
    Row::new()
        .push(questions_list_container)
        .push(question_with_controls)
        .into()
}

fn questions_list_container(questions_list: Elem<'_>) -> Elem<'_> {
    Container::new(questions_list)
        .height(Length::Fill)
        .width(Length::Units(150))
        .style(style::QuestionsColumn)
        .center_y()
        .center_x()
        .into()
}

fn question_with_controls_column<'a>(question_view: Elem<'a>, controls_row: Elem<'a>) -> Elem<'a> {
    Column::new()
        .padding(25)
        .width(Length::FillPortion(9))
        .push(question_view)
        .push(Space::with_height(Length::Fill))
        .push(controls_row)
        .into()
}

fn controls_row(ctrls: Buttons<'_>) -> Elem<'_> {
    Row::new()
        .height(Length::Units(50))
        .push(ctrls.0)
        .push(Space::with_width(Length::Fill))
        .push(ctrls.1)
        .into()
}

pub(crate) fn build_settings_view<'a>(
    settings_list: Elem<'a>,
    header: Elem<'a>,
    questions_view: Elem<'a>,
) -> Elem<'a> {
    let header = Row::new()
        .height(Length::FillPortion(4))
        .push(Space::with_width(Length::Fill))
        .push(header);

    let question = Column::new()
        .height(Length::FillPortion(70))
        .push(questions_view);

    let main_view: Elem<'a> = Column::new().padding(25).push(header).push(question).into();

    debug(Row::new().push(settings_list).push(main_view).into())
}

fn debug(element: Element<'_, Msg>) -> Element<'_, Msg> {
    let mut element = element;
    match env::var("EXPLAIN_LAYOUT") {
        Ok(string) => {
            if string == "true" {
                element = element.explain(Color::BLACK);
                element
            } else {
                element
            }
        }
        _ => element,
    }
}

pub(crate) fn results_view(content: Elem<'_>) -> Elem<'_> {
    let question_with_controls = Column::new()
        .width(Length::Fill)
        .spacing(20)
        .padding(20)
        .push(content)
        .push(Space::with_height(Length::Fill));

    Container::new(question_with_controls)
        .height(Length::Fill)
        .width(Length::Fill)
        .center_x()
        .center_y()
        .into()
}

pub(crate) fn button<'a, Message>(
    state: &'a mut button::State,
    label: &str,
) -> Button<'a, Message> {
    Button::new(
        state,
        Text::new(label).horizontal_alignment(HorizontalAlignment::Center),
    )
    .padding(12)
    .min_width(150)
    .style(style::Button)
}

pub(crate) fn settings_button<'a, Message>(
    state: &'a mut button::State,
    label: &str,
) -> Button<'a, Message> {
    Button::new(
        state,
        Text::new(label).horizontal_alignment(HorizontalAlignment::Center),
    )
    .min_width(35)
    .style(style::SettingsButton)
}

pub(crate) fn radio<'a>(
    answer_idx: usize,
    answer_text: &str,
    selected_answer: Option<usize>,
) -> Elem<'a> {
    Radio::new(answer_idx, answer_text, selected_answer, Msg::Answer)
        .style(style::Radio)
        .into()
}

pub(crate) fn checkbox<'a>(is_checked: bool, answer_text: &str, answer_idx: usize) -> Elem<'a> {
    Checkbox::new(is_checked, answer_text, move |_| Msg::Answer(answer_idx))
        .size(25)
        .style(style::Checkbox)
        .into()
}

pub(crate) fn listing_label<'a, Message>(
    state: &'a mut button::State,
    label: &str,
) -> Button<'a, Message> {
    Button::new(
        state,
        Text::new(label).horizontal_alignment(HorizontalAlignment::Center),
    )
    .padding(12)
    .width(Length::Fill)
    .style(style::QuestionLabel)
}

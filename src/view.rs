use crate::controls::Controls;
use crate::header::Header;
use crate::question::Questions;
use crate::question_list::QuestionList;
use crate::quizers::Msg;
use crate::settings::Settings;
use crate::settings_list::SettingsList;
use crate::style;
use conv::prelude::*;
use iced::alignment::{Horizontal, Vertical};
use iced::{
    button, Button, Checkbox, Color, Column, Container, Element, Length, Radio, Row, Space, Text,
};
use std::env;

pub(crate) type Elem<'a> = Element<'a, Msg>;
pub(crate) type Buttons<'a> = (Button<'a, Msg>, Button<'a, Msg>);

pub(crate) enum Page {
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
    pub(crate) current_page: Page,
    show_results: bool,
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
            current_page: Page::Question,
            show_results: false,
        }
    }

    pub(crate) fn question(&mut self) -> Elem<'_> {
        let questions_list = if self.show_results {
            self.questions_list
                .revealed(&self.questions.answers_state())
        } else {
            self.questions_list.view(self.page_idx)
        };
        build_main_view(
            questions_list,
            self.header.view(),
            self.questions[self.page_idx].view(),
            self.controls.ctrls(self.page_idx),
        )
    }

    pub(crate) fn results(&mut self) -> Elem<'_> {
        self.show_results = true;
        let result = format_result_msg(&self.questions);
        let results_section = Column::new().spacing(20).push(Text::new(result));
        let questions_list = if self.show_results {
            self.questions_list
                .revealed(&self.questions.answers_state())
        } else {
            self.questions_list.view(self.page_idx)
        };
        build_main_view(
            questions_list,
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

    pub(crate) fn go_results_page(&mut self) {
        self.page_idx = self.questions.count();
    }

    fn update_current_page(&mut self) {
        self.current_page = match self.page_idx {
            x if x < self.questions.count() => Page::Question,
            x if x == self.questions.count() => Page::Results,
            x if x == self.questions.count() + 1 => Page::Settings,
            _ => panic!("no such page"),
        }
    }

    pub(crate) fn current(&mut self) -> Elem<'_> {
        self.update_current_page();
        match &mut self.current_page {
            Page::Question => self.question(),
            Page::Results => self.results(),
            Page::Settings => self.settings(),
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
        f64::value_from(points).expect("failed to convert from usize to f64") * 100.0
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
                list_container(questions_list),
                main_view_column(question_view, Some(controls_row(buttons))),
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

fn list_container(questions_list: Elem<'_>) -> Elem<'_> {
    Container::new(questions_list)
        .height(Length::Fill)
        .width(Length::Units(150))
        .style(style::QuestionsColumn)
        .center_y()
        .center_x()
        .into()
}

fn main_view_column<'a>(question_view: Elem<'a>, controls_row: Option<Elem<'a>>) -> Elem<'a> {
    let mut column = Column::new()
        .padding(25)
        .width(Length::FillPortion(9))
        .push(question_view)
        .push(Space::with_height(Length::Fill));

    if let Some(controls_row) = controls_row {
        column = column.push(controls_row);
    }

    column.into()
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
    setting_view: Elem<'a>,
) -> Elem<'a> {
    debug(
        Column::new()
            .push(control_bar_container(header))
            .push(quiz_view_row(
                list_container(settings_list),
                main_view_column(setting_view, None),
            ))
            .into(),
    )
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
        Err(_) => element,
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

pub(crate) fn button<'a, Message: Clone>(
    state: &'a mut button::State,
    label: &str,
) -> Button<'a, Message> {
    Button::new(
        state,
        Text::new(label).horizontal_alignment(Horizontal::Center),
    )
    .padding(12)
    .width(Length::Units(150))
    .style(style::Button)
}

pub(crate) fn header_button<'a, Message: Clone>(
    state: &'a mut button::State,
    label: &str,
) -> Button<'a, Message> {
    Button::new(
        state,
        Text::new(label).horizontal_alignment(Horizontal::Center),
    )
    .width(Length::Units(35))
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

pub(crate) fn listing_label<'a, Message: Clone>(
    state: &'a mut button::State,
    label: &str,
) -> Button<'a, Message> {
    Button::new(
        state,
        Text::new(label)
            .horizontal_alignment(Horizontal::Center)
            .vertical_alignment(Vertical::Center),
    )
    .padding(12)
    .width(Length::Fill)
    .height(Length::Units(50))
    .style(style::QuestionLabel)
}

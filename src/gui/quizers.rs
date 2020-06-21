use crate::gui::question_view::{QuestionMsg, QuestionsView};
use crate::gui::State;
use iced::{
    button, scrollable, Button, Column, Container, Element, HorizontalAlignment, Length, Row,
    Sandbox, Scrollable, Space, Text,
};
use md_questions::Questions;
use std::fs::read_to_string;

#[derive(Debug, Clone)]
pub enum Msg {
    BackPressed,
    NextPressed,
    FinishPressed,
    RestartPressed,
    Answer(QuestionMsg),
}

pub struct Quizers {
    questions: QuestionsView,
    scroll: scrollable::State,
    back_button: button::State,
    next_button: button::State,
    restart_button: button::State,
    state: State,
}

impl Sandbox for Quizers {
    type Message = Msg;

    fn new() -> Quizers {
        let content = read_to_string("/home/zbychu/projects/md-questions/res/QUESTIONS.md")
            .expect("failed to read questions markdown");
        let questions_provider = Questions::from(content.as_str());
        let number_of_questions = questions_provider.len();
        Quizers {
            questions: QuestionsView::new(Box::new(questions_provider)),
            scroll: scrollable::State::new(),
            back_button: button::State::new(),
            next_button: button::State::new(),
            restart_button: button::State::new(),
            state: State::new(number_of_questions),
        }
    }

    fn title(&self) -> String {
        "Quizers".into()
    }

    fn update(&mut self, event: Msg) {
        match event {
            Msg::BackPressed => self.state.go_back(),
            Msg::NextPressed => self.state.advance(),
            Msg::Answer(QuestionMsg::Answered(selected)) => {
                self.state.selected_answer = Some(selected)
            }
            Msg::FinishPressed => self.state.show_results = true,
            Msg::RestartPressed => self.state = State::new(self.state.number_of_questions),
        }
    }

    fn view(&mut self) -> Element<Msg> {
        let mut controls = Row::new();

        if self.state.has_previous() && !self.state.show_results {
            controls = controls.push(
                button(&mut self.back_button, "Back")
                    .on_press(Msg::BackPressed)
                    .style(style::Button::Secondary),
            );
        }

        controls = controls.push(Space::with_width(Length::Fill));

        if self.state.can_continue() {
            controls =
                controls.push(button(&mut self.next_button, "Next").on_press(Msg::NextPressed));
        } else if !self.state.should_show_results() {
            controls =
                controls.push(button(&mut self.next_button, "Finish").on_press(Msg::FinishPressed));
        } else if self.state.should_show_results() {
            controls = controls
                .push(button(&mut self.restart_button, "Restart").on_press(Msg::RestartPressed));
        }

        let content: Element<_> = Column::new()
            .max_width(540)
            .spacing(20)
            .padding(20)
            .push(self.questions.view(&self.state).map(Msg::Answer))
            .push(controls)
            .into();

        let scrollable = Scrollable::new(&mut self.scroll)
            .push(Container::new(content).width(Length::Fill).center_x());

        Container::new(scrollable)
            .height(Length::Fill)
            .center_y()
            .style(style::Container)
            .into()
    }
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
    .min_width(100)
    .style(style::Button::Primary)
}

pub(crate) mod style {
    use iced::{button, container, radio, Background, Color, Vector};

    pub enum Button {
        Primary,
        Secondary,
    }

    const ACTIVE: Color = Color::from_rgb(
        0x72 as f32 / 255.0,
        0x89 as f32 / 255.0,
        0xDA as f32 / 255.0,
    );

    const SURFACE: Color = Color::from_rgb(
        0x40 as f32 / 255.0,
        0x44 as f32 / 255.0,
        0x4B as f32 / 255.0,
    );

    pub struct Radio;

    impl radio::StyleSheet for Radio {
        fn active(&self) -> radio::Style {
            radio::Style {
                background: Background::Color(SURFACE),
                dot_color: ACTIVE,
                border_width: 1,
                border_color: ACTIVE,
            }
        }

        fn hovered(&self) -> radio::Style {
            radio::Style {
                background: Background::Color(Color { a: 0.5, ..SURFACE }),
                ..self.active()
            }
        }
    }

    pub struct Container;

    impl container::StyleSheet for Container {
        fn style(&self) -> container::Style {
            container::Style {
                background: Some(Background::Color(Color::from_rgb8(0x36, 0x39, 0x3F))),
                text_color: Some(Color::WHITE),
                ..container::Style::default()
            }
        }
    }

    impl button::StyleSheet for Button {
        fn active(&self) -> button::Style {
            button::Style {
                background: Some(Background::Color(ACTIVE)),
                border_radius: 3,
                text_color: Color::WHITE,
                ..button::Style::default()
            }
        }

        fn hovered(&self) -> button::Style {
            button::Style {
                text_color: Color::WHITE,
                shadow_offset: Vector::new(1.0, 2.0),
                ..self.active()
            }
        }
    }
}

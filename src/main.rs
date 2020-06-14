use iced::{
    button, scrollable, Button, Column, Container, Element, HorizontalAlignment, Length, Radio,
    Row, Sandbox, Scrollable, Settings, Space, Text,
};
use md_questions::{Question, Questions};
use question::QuestionsProvider;
use std::fs::read_to_string;

mod question;

fn main() {
    pretty_env_logger::init();
    Quizer::run(Settings::default())
}

pub struct Quizer {
    questions: QuestionsView,
    scroll: scrollable::State,
    back_button: button::State,
    next_button: button::State,
}

impl Sandbox for Quizer {
    type Message = Message;

    fn new() -> Quizer {
        let content = read_to_string("/home/zbychu/projects/md-questions/res/QUESTIONS.md")
            .expect("failed to read questions markdown");
        Quizer {
            questions: QuestionsView::new(Box::new(Questions::from(content.as_str()))),
            scroll: scrollable::State::new(),
            back_button: button::State::new(),
            next_button: button::State::new(),
        }
    }

    fn title(&self) -> String {
        "Quizers".into()
    }

    fn update(&mut self, event: Message) {
        match event {
            Message::BackPressed => {
                self.questions.go_back();
            }
            Message::NextPressed => {
                self.questions.advance();
            }
            Message::StepMessage(step_msg) => {
                self.questions.update(step_msg);
            }
        }
    }

    fn view(&mut self) -> Element<Message> {
        let Quizer {
            questions,
            scroll,
            back_button,
            next_button,
            ..
        } = self;

        let mut controls = Row::new();

        if questions.has_previous() {
            controls = controls.push(
                button(back_button, "Back")
                    .on_press(Message::BackPressed)
                    .style(style::Button::Secondary),
            );
        }

        controls = controls.push(Space::with_width(Length::Fill));

        if questions.can_continue() {
            controls = controls.push(
                button(next_button, "Next")
                    .on_press(Message::NextPressed)
                    .style(style::Button::Primary),
            );
        }

        let content: Element<_> = Column::new()
            .max_width(540)
            .spacing(20)
            .padding(20)
            .push(questions.view().map(Message::StepMessage))
            .push(controls)
            .into();

        let scrollable =
            Scrollable::new(scroll).push(Container::new(content).width(Length::Fill).center_x());

        Container::new(scrollable)
            .height(Length::Fill)
            .center_y()
            .style(style::Container)
            .into()
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    BackPressed,
    NextPressed,
    StepMessage(QuestionMessage),
}

struct QuestionsView {
    questions_provider: Box<dyn QuestionsProvider>,
    current: usize,
    selected_answer: usize,
}

impl QuestionsView {
    fn new(questions_provider: Box<dyn QuestionsProvider>) -> QuestionsView {
        QuestionsView {
            questions_provider,
            current: 0,
            selected_answer: 0,
        }
    }

    fn update(&mut self, msg: QuestionMessage) {
        self.update_view(msg);
    }

    fn update_view(&mut self, msg: QuestionMessage) {
        match msg {
            QuestionMessage::Answered(selected) => {
                self.selected_answer = selected;
            }
        };
    }

    fn container<'a>(title: &str) -> Column<'a, QuestionMessage> {
        Column::new().spacing(20).push(Text::new(title).size(50))
    }

    fn view(&mut self) -> Element<QuestionMessage> {
        Self::radio(
            &self.questions_provider.question(self.current),
            self.selected_answer,
        )
        .into()
    }

    fn radio<'a>(question: &Question, selected_answer: usize) -> Column<'a, QuestionMessage> {
        let q = Column::new()
            .padding(20)
            .spacing(10)
            .push(Text::new("Iced is written in...").size(24))
            .push((0..question.no_answers()).fold(
                Column::new().padding(10).spacing(20),
                |choices, answer| {
                    choices.push(
                        Radio::new(
                            answer,
                            &question.answer(answer).text(),
                            Some(selected_answer),
                            QuestionMessage::Answered,
                        )
                        .style(style::Radio),
                    )
                },
            ));

        Self::container("Radio button")
            .push(Text::new(&question.text()))
            .push(q)
            .push(Text::new(
                "Iced works very well with iterators! The list above is \
                 basically created by folding a column over the different \
                 choices, creating a radio button for each one of them!",
            ))
    }

    fn advance(&mut self) {
        if self.can_continue() {
            self.current += 1;
        }
    }

    fn go_back(&mut self) {
        if self.has_previous() {
            self.current -= 1;
        }
    }

    fn has_previous(&self) -> bool {
        self.current > 0
    }

    fn can_continue(&self) -> bool {
        self.current + 1 < self.questions_provider.len()
    }
}

#[derive(Debug, Clone)]
pub enum QuestionMessage {
    Answered(usize),
}

fn button<'a, Message>(state: &'a mut button::State, label: &str) -> Button<'a, Message> {
    Button::new(
        state,
        Text::new(label).horizontal_alignment(HorizontalAlignment::Center),
    )
    .padding(12)
    .min_width(100)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Answers {
    First,
    Second,
    Third,
    Fourth,
    Fifth,
}

impl From<Answers> for usize {
    fn from(answer: Answers) -> usize {
        match answer {
            Answers::First => 0,
            Answers::Second => 1,
            Answers::Third => 2,
            Answers::Fourth => 3,
            Answers::Fifth => 4,
        }
    }
}

mod style {
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

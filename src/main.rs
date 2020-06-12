use iced::{
    button, scrollable, Button, Column, Container, Element, HorizontalAlignment, Length, Radio,
    Row, Sandbox, Scrollable, Settings, Space, Text,
};
use log::debug;

fn main() {
    pretty_env_logger::init();
    Tour::run(Settings::default())
}

pub struct Tour {
    steps: Steps,
    scroll: scrollable::State,
    back_button: button::State,
    next_button: button::State,
}

impl Sandbox for Tour {
    type Message = Message;

    fn new() -> Tour {
        Tour {
            steps: Steps::new(),
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
                self.steps.go_back();
            }
            Message::NextPressed => {
                self.steps.advance();
            }
            Message::StepMessage(step_msg) => {
                self.steps.update(step_msg);
            }
        }
    }

    fn view(&mut self) -> Element<Message> {
        let Tour {
            steps,
            scroll,
            back_button,
            next_button,
            ..
        } = self;

        let mut controls = Row::new();

        if steps.has_previous() {
            controls = controls.push(
                button(back_button, "Back")
                    .on_press(Message::BackPressed)
                    .style(style::Button::Secondary),
            );
        }

        controls = controls.push(Space::with_width(Length::Fill));

        if steps.can_continue() {
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
            .push(steps.view().map(Message::StepMessage))
            .push(controls)
            .into();

        let scrollable =
            Scrollable::new(scroll).push(Container::new(content).width(Length::Fill).center_x());

        Container::new(scrollable)
            .height(Length::Fill)
            .center_y()
            .into()
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    BackPressed,
    NextPressed,
    StepMessage(StepMessage),
}

struct Steps {
    steps: Vec<Step>,
    current: usize,
}

impl Steps {
    fn new() -> Steps {
        Steps {
            steps: vec![Step::Radio { selection: None }, Step::End],
            current: 0,
        }
    }

    fn update(&mut self, msg: StepMessage) {
        self.steps[self.current].update(msg);
    }

    fn view(&mut self) -> Element<StepMessage> {
        self.steps[self.current].view()
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
        self.current + 1 < self.steps.len() && self.steps[self.current].can_continue()
    }

    fn title(&self) -> &str {
        self.steps[self.current].title()
    }
}

enum Step {
    Radio { selection: Option<Language> },
    End,
}

#[derive(Debug, Clone)]
pub enum StepMessage {
    LanguageSelected(Language),
}

impl<'a> Step {
    fn update(&mut self, msg: StepMessage) {
        match msg {
            StepMessage::LanguageSelected(language) => {
                if let Step::Radio { selection } = self {
                    *selection = Some(language);
                }
            }
        };
    }

    fn title(&self) -> &str {
        match self {
            Step::Radio { .. } => "Radio button",
            Step::End => "End",
        }
    }

    fn can_continue(&self) -> bool {
        match self {
            Step::Radio { selection } => *selection == Some(Language::Rust),
            Step::End => false,
        }
    }

    fn view(&mut self) -> Element<StepMessage> {
        match self {
            Step::Radio { selection } => Self::radio(*selection),
            Step::End => Self::end(),
        }
        .into()
    }

    fn radio(selection: Option<Language>) -> Column<'a, StepMessage> {
        let question = Column::new()
            .padding(20)
            .spacing(10)
            .push(Text::new("Iced is written in...").size(24))
            .push(Language::all().iter().cloned().fold(
                Column::new().padding(10).spacing(20),
                |choices, language| {
                    choices.push(Radio::new(
                        language,
                        language,
                        selection,
                        StepMessage::LanguageSelected,
                    ))
                },
            ));

        Self::container("Radio button")
            .push(Text::new(
                "A radio button is normally used to represent a choice... \
                 Surprise test!",
            ))
            .push(question)
            .push(Text::new(
                "Iced works very well with iterators! The list above is \
                 basically created by folding a column over the different \
                 choices, creating a radio button for each one of them!",
            ))
    }

    fn container(title: &str) -> Column<'a, StepMessage> {
        Column::new().spacing(20).push(Text::new(title).size(50))
    }

    fn end() -> Column<'a, StepMessage> {
        Self::container("You reached the end!")
            .push(Text::new(
                "This tour will be updated as more features are added.",
            ))
            .push(Text::new("Make sure to keep an eye on it!"))
    }
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
pub enum Language {
    Rust,
    Elm,
    Ruby,
    Haskell,
    C,
    Other,
}

impl Language {
    fn all() -> [Language; 6] {
        [
            Language::C,
            Language::Elm,
            Language::Ruby,
            Language::Haskell,
            Language::Rust,
            Language::Other,
        ]
    }
}

impl From<Language> for String {
    fn from(language: Language) -> String {
        String::from(match language {
            Language::Rust => "Rust",
            Language::Elm => "Elm",
            Language::Ruby => "Ruby",
            Language::Haskell => "Haskell",
            Language::C => "C",
            Language::Other => "Other",
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Layout {
    Row,
    Column,
}

mod style {
    use iced::{button, Background, Color, Vector};

    pub enum Button {
        Primary,
        Secondary,
    }

    const ACTIVE: Color = Color::from_rgb(
        0x72 as f32 / 255.0,
        0x89 as f32 / 255.0,
        0xDA as f32 / 255.0,
    );

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

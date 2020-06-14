use crate::gui::question_view::{QuestionMessage, QuestionsView};
use crate::gui::style;
use iced::{
    button, scrollable, Button, Column, Container, Element, HorizontalAlignment, Length, Row,
    Sandbox, Scrollable, Space, Text,
};
use md_questions::Questions;
use std::fs::read_to_string;

#[derive(Debug, Clone)]
pub enum Message {
    BackPressed,
    NextPressed,
    Step(QuestionMessage),
}

pub struct Quizers {
    questions: QuestionsView,
    scroll: scrollable::State,
    back_button: button::State,
    next_button: button::State,
}

impl Sandbox for Quizers {
    type Message = Message;

    fn new() -> Quizers {
        let content = read_to_string("/home/zbychu/projects/md-questions/res/QUESTIONS.md")
            .expect("failed to read questions markdown");
        Quizers {
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
            Message::Step(step_msg) => {
                self.questions.update(&step_msg);
            }
        }
    }

    fn view(&mut self) -> Element<Message> {
        let Quizers {
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
            .push(questions.view().map(Message::Step))
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

fn button<'a, Message>(state: &'a mut button::State, label: &str) -> Button<'a, Message> {
    Button::new(
        state,
        Text::new(label).horizontal_alignment(HorizontalAlignment::Center),
    )
    .padding(12)
    .min_width(100)
}

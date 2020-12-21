use iced::{button, checkbox, container, radio, Background, Color, Vector};

const ACTIVE: Color = Color::from_rgb(32_f32 / 255.0, 90_f32 / 255.0, 151_f32 / 255.0);

const CORRECT_ANSWER: Color = Color::from_rgb(116_f32 / 255.0, 179_f32 / 255.0, 128_f32 / 255.0);

const WRONG_ANSWER: Color = Color::from_rgb(205_f32 / 255.0, 64_f32 / 255.0, 62_f32 / 255.0);

const SURFACE: Color = Color::from_rgb(48_f32 / 255.0, 55_f32 / 255.0, 57_f32 / 255.0);

const HEADER_SURFACE: Color = Color::from_rgb(34_f32 / 255.0, 39_f32 / 255.0, 41_f32 / 255.0);

const SIDEBAR_SURFACE: Color = Color::from_rgb(43_f32 / 255.0, 51_f32 / 255.0, 54_f32 / 255.0);

pub struct Radio;

impl radio::StyleSheet for Radio {
    fn active(&self) -> radio::Style {
        radio::Style {
            background: Background::Color(SURFACE),
            dot_color: ACTIVE,
            border_width: 1.0,
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

pub struct Header;

impl container::StyleSheet for Header {
    fn style(&self) -> container::Style {
        container::Style {
            background: Some(Background::Color(HEADER_SURFACE)),
            text_color: Some(Color::WHITE),
            ..container::Style::default()
        }
    }
}

pub struct QuestionsColumn;

impl container::StyleSheet for QuestionsColumn {
    fn style(&self) -> container::Style {
        container::Style {
            background: Some(Background::Color(SIDEBAR_SURFACE)),
            text_color: Some(Color::WHITE),
            ..container::Style::default()
        }
    }
}

pub struct Button;

impl button::StyleSheet for Button {
    fn active(&self) -> button::Style {
        button::Style {
            background: Some(Background::Color(ACTIVE)),
            border_radius: 3.0,
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

pub struct QuestionLabel;

impl button::StyleSheet for QuestionLabel {
    fn active(&self) -> button::Style {
        button::Style {
            background: Some(Background::Color(SIDEBAR_SURFACE)),
            border_radius: 0.0,
            text_color: Color::WHITE,
            ..button::Style::default()
        }
    }

    fn hovered(&self) -> button::Style {
        self.active()
    }
}

pub struct SelectedLabel;

impl button::StyleSheet for SelectedLabel {
    fn active(&self) -> button::Style {
        button::Style {
            background: Some(Background::Color(ACTIVE)),
            border_radius: 0.0,
            text_color: Color::WHITE,
            ..button::Style::default()
        }
    }

    fn hovered(&self) -> button::Style {
        self.active()
    }
}

pub struct CorrectAnswer;

impl button::StyleSheet for CorrectAnswer {
    fn active(&self) -> button::Style {
        button::Style {
            background: Some(Background::Color(CORRECT_ANSWER)),
            border_radius: 0.0,
            text_color: Color::WHITE,
            ..button::Style::default()
        }
    }

    fn hovered(&self) -> button::Style {
        self.active()
    }
}

pub struct WrongAnswer;

impl button::StyleSheet for WrongAnswer {
    fn active(&self) -> button::Style {
        button::Style {
            background: Some(Background::Color(WRONG_ANSWER)),
            border_radius: 0.0,
            text_color: Color::WHITE,
            ..button::Style::default()
        }
    }

    fn hovered(&self) -> button::Style {
        self.active()
    }
}

pub struct Checkbox;

impl checkbox::StyleSheet for Checkbox {
    fn active(&self, _: bool) -> checkbox::Style {
        checkbox::Style {
            background: Background::Color(SURFACE),
            checkmark_color: ACTIVE,
            border_width: 1.0,
            border_color: ACTIVE,
            border_radius: 3.0,
        }
    }

    fn hovered(&self, is_checked: bool) -> checkbox::Style {
        checkbox::Style {
            background: Background::Color(Color { a: 0.5, ..SURFACE }),
            ..self.active(is_checked)
        }
    }
}

pub struct SettingsButton;

impl button::StyleSheet for SettingsButton {
    fn active(&self) -> button::Style {
        button::Style {
            background: Some(Background::Color(SURFACE)),
            border_radius: 10.0,
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

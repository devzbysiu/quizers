pub(crate) mod helpers;
pub(crate) mod quizers;
pub(crate) mod view;

pub(crate) mod style {
    use iced::{button, container, radio, Background, Color, Vector};

    const ACTIVE: Color = Color::from_rgb(114_f32 / 255.0, 137_f32 / 255.0, 218_f32 / 255.0);

    const SURFACE: Color = Color::from_rgb(64_f32 / 255.0, 68_f32 / 255.0, 75_f32 / 255.0);

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

    pub struct QuestionsColumn;

    impl container::StyleSheet for QuestionsColumn {
        fn style(&self) -> container::Style {
            container::Style {
                background: Some(Background::Color(Color::from_rgb8(82, 84, 87))),
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

    pub struct QuestionLabel;

    impl button::StyleSheet for QuestionLabel {
        fn active(&self) -> button::Style {
            button::Style {
                background: Some(Background::Color(Color::from_rgb8(82, 84, 87))),
                border_radius: 0,
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

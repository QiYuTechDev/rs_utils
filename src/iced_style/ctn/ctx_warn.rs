use iced::container::{Style, StyleSheet};
use iced::{Background, Color};

#[derive(Default)]
pub struct CtxWarn {}

impl StyleSheet for CtxWarn {
    fn style(&self) -> Style {
        Style {
            text_color: Some(Color::from_rgb8(148, 118, 0)),
            background: Some(Background::Color(Color::from_rgb8(255, 251, 235))),
            border_width: 0f32,
            border_radius: 5f32,
            border_color: Color::TRANSPARENT,
        }
    }
}

use iced::container::{Style, StyleSheet};
use iced::{Background, Color};

#[derive(Default)]
pub struct CtxInfo {}

impl StyleSheet for CtxInfo {
    fn style(&self) -> Style {
        Style {
            text_color: Some(Color::from_rgb8(0, 148, 126)),
            background: Some(Background::Color(Color::from_rgb8(235, 255, 252))),
            border_width: 0f32,
            border_radius: 5f32,
            border_color: Color::TRANSPARENT,
        }
    }
}

use iced::container::{Style, StyleSheet};
use iced::{Background, Color};

#[derive(Default)]
pub struct CtxSuccess {}

impl StyleSheet for CtxSuccess {
    fn style(&self) -> Style {
        Style {
            text_color: Some(Color::from_rgb8(37, 121, 66)),
            background: Some(Background::Color(Color::from_rgb8(239, 250, 243))),
            border_width: 0f32,
            border_radius: 5f32,
            border_color: Color::TRANSPARENT,
        }
    }
}

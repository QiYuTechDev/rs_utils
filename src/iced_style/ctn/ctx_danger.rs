use iced::container::{Style, StyleSheet};
use iced::{Background, Color};

#[derive(Default)]
pub struct CtxDanger {}

impl StyleSheet for CtxDanger {
    fn style(&self) -> Style {
        Style {
            text_color: Some(Color::from_rgb8(217, 73, 102)),
            background: Some(Background::Color(Color::from_rgb8(254, 236, 240))),
            border_width: 0f32,
            border_radius: 5f32,
            border_color: Color::TRANSPARENT,
        }
    }
}

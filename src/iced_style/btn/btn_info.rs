use iced::button::Style;
use iced::{button, Background, Color, Vector};

/// 蓝色漂亮的按钮 (success)
#[derive(Default)]
pub struct ButtonInfoColor {}

impl iced::button::StyleSheet for ButtonInfoColor {
    fn active(&self) -> button::Style {
        button::Style {
            background: Some(Background::Color(Color::from_rgb8(50, 152, 220))),
            border_radius: 5.0,
            text_color: Color::WHITE,
            shadow_offset: Vector::new(1.0, 1.0),
            ..button::Style::default()
        }
    }

    fn hovered(&self) -> Style {
        self.active()
    }

    fn pressed(&self) -> Style {
        self.active()
    }

    fn disabled(&self) -> Style {
        self.active()
    }
}

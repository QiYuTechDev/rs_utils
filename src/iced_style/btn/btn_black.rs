use iced::button::Style;
use iced::{button, Background, Color, Vector};

/// 蓝色漂亮的按钮 (success)
#[derive(Default)]
pub struct ButtonBlackColor {}

impl iced::button::StyleSheet for ButtonBlackColor {
    fn active(&self) -> button::Style {
        button::Style {
            shadow_offset: Vector::new(0.0, 0.0),
            background: Some(Background::Color(Color::BLACK)),
            border_radius: 5.0,
            border_width: 0.0,
            border_color: Color::BLACK,
            text_color: Color::WHITE,
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

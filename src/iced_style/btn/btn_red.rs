use iced::button::Style;
use iced::{button, Background, Color};

/// 红色漂亮的按钮 (failure) 失败 警告等
#[derive(Default)]
pub struct ButtonRedColor {}

impl iced::button::StyleSheet for ButtonRedColor {
    fn active(&self) -> button::Style {
        button::Style {
            background: Some(Background::Color(Color::from_rgb8(241, 70, 104))),
            border_radius: 5.0,
            text_color: Color::WHITE,
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

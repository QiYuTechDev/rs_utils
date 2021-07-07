use iced::button::Style;
use iced::{Background, Color};

/// 红色漂亮的按钮 (failure) 失败 警告等
#[derive(Default)]
pub struct ButtonWarnColor {}

impl iced::button::StyleSheet for ButtonWarnColor {
    fn active(&self) -> Style {
        Style {
            background: Some(Background::Color(Color::from_rgb8(255, 221, 87))),
            border_radius: 5.0,
            text_color: Color::BLACK,
            ..Style::default()
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

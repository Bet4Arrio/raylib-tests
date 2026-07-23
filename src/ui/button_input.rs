use crate::core::traits::Component;
use sola_raylib::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ButtonStatus {
    Idle,
    Hover,
    Clicked,
}

pub struct ButtonInput {
    rect: Rectangle,
    text: String,
    color: Color,
    status: ButtonStatus,
}

impl ButtonInput {
    pub fn new(rect: Rectangle, text: String) -> Self {
        ButtonInput {
            rect,
            text: text,
            color: Color::BLACK,
            status: ButtonStatus::Idle,
        }
    }
    pub fn is_clicked(&self) -> bool {
        self.status == ButtonStatus::Clicked
    }
}

impl Component for ButtonInput {
    fn check(self: &mut Self, rl: &mut RaylibHandle) {
        let mouse_pos = rl.get_mouse_position();
        if self.rect.check_collision_point_rec(mouse_pos) {
            if rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) {
                self.status = ButtonStatus::Clicked;
            } else {
                self.status = ButtonStatus::Hover;
            }
        } else {
            self.status = ButtonStatus::Idle;
        }
    }

    fn draw(self: &Self, d: &mut RaylibDrawHandle) {
        let color_bg = match self.status {
            ButtonStatus::Idle => Color::DARKPURPLE,
            _ => Color::WHITE,
        };

        let color_txt = match self.status {
            ButtonStatus::Idle => Color::WHITE,
            _ => Color::DARKPURPLE,
        };
        d.draw_rectangle_rounded(self.rect, 0.0, 1, color_bg);
        let text_width = d.measure_text(&self.text, 12);
        d.draw_text(
            &self.text,
            self.rect.x as i32 + (self.rect.width as i32 - text_width) / 2,
            self.rect.y as i32 + (self.rect.height as i32 - 12) / 2,
            12,
            color_txt,
        );
    }
}

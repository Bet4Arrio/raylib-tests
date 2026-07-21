use crate::core::traits::Component;
use sola_raylib::prelude::*;
use std::char;

pub struct TextInput<T> {
    rect: Rectangle,
    selected: bool,
    val: T,
}

pub trait RaylibTextInput {
    fn get_next_value(&self, current: Self, rl: &mut RaylibHandle) -> Self;
}
impl RaylibTextInput for i32 {
    fn get_next_value(&self, current: i32, rl: &mut RaylibHandle) -> i32 {
        if rl.is_key_pressed(KeyboardKey::KEY_BACKSPACE) {
            return current / 10;
        }
        let character = rl.get_char_pressed();
        if let Some(c) = character {
            let plus = match c {
                '1' => 1,
                '2' => 2,
                '3' => 3,
                '4' => 4,
                '5' => 5,
                '6' => 6,
                '7' => 7,
                '8' => 8,
                '9' => 9,
                '0' => 0,
                _ => -1,
            };
            print!("pressed : {c}");
            if plus < 0 {
                return current;
            }
            return (current * 10) + plus;
        }
        current
    }
}

impl<T: Copy + RaylibTextInput + ToString> TextInput<T> {
    pub fn update_value(self: &mut Self, rl: &mut RaylibHandle) {
        if self.check_click(rl) {
            self.val = self.val.get_next_value(self.val, rl);
        }
    }
    pub fn new(rec: Rectangle, value: T) -> Self {
        TextInput {
            rect: rec,
            selected: false,
            val: value,
        }
    }
    pub fn get_val(self: &Self) -> T {
        self.val
    }

    pub fn check_click(self: &mut Self, rl: &mut RaylibHandle) -> bool {
        let mouse_pos = rl.get_mouse_position();
        if rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) {
            if self.rect.check_collision_point_rec(mouse_pos) {
                self.selected = !self.selected;
            } else {
                self.selected = false;
            }
        }
        self.selected
    }
}

impl<T: Copy + RaylibTextInput + ToString> Component for TextInput<T> {
    fn check(&mut self, rl: &mut RaylibHandle) {
        let mouse_pos = rl.get_mouse_position();
        if rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) {
            if self.rect.check_collision_point_rec(mouse_pos) {
                self.selected = !self.selected;
            } else {
                self.selected = false;
            }
        }
    }

    fn draw(self: &Self, d: &mut RaylibDrawHandle) {
        if self.selected {
            d.draw_rectangle_rounded_lines(self.rect, 0.0, 1, Color::BLACK);
        } else {
            d.draw_rectangle_rounded_lines(self.rect, 0.0, 1, Color::PINK);
        }
        let (x, y, h) = (
            self.rect.x as i32,
            self.rect.y as i32,
            self.rect.height as i32,
        );

        d.draw_text(&self.val.to_string(), x, y, h, Color::PINK)
    }
}

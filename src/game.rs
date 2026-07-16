use std::hash;

use crate::life::{GameOfLife, GameOfLifeSeter};
use sola_raylib::prelude::*;

enum SelectedGame {
    Life(GameOfLife),
    LifeSeter(GameOfLifeSeter),
    None,
}
pub struct GameHub {
    select_game: SelectedGame,
}

fn update_number(actual: i32, delete: bool, charecter: Option<char>) -> i32 {
    if !delete && charecter.is_none() {
        return actual;
    }
    if delete {
        return actual / 10;
    }
    if let Some(c) = charecter {
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
            return actual;
        }
        return (actual * 10) + plus;
    }
    actual
}
impl GameHub {
    pub fn init() -> Self {
        GameHub {
            select_game: SelectedGame::None,
        }
    }
    fn handle_keypress(
        self: &mut Self,
        button: KeyboardKey,
        rl: &mut RaylibHandle,
        mut _thread: &RaylibThread,
    ) {
        match button {
            KeyboardKey::KEY_Q => {
                rl.request_quit();
            }
            KeyboardKey::KEY_L => {
                self.init_life();
            }
            _ => {}
        }
    }
    fn init_life(self: &mut Self) {
        if let SelectedGame::LifeSeter(seter) = &self.select_game {
            self.select_game = SelectedGame::Life(GameOfLife::init(
                50,
                60,
                seter.h_input.get_val(),
                seter.w_input.get_val(),
            ))
        } else {
            self.select_game = SelectedGame::LifeSeter(GameOfLifeSeter::default())
        }
    }

    fn run_hub(self: &mut Self, rl: &mut RaylibHandle, thread: &RaylibThread) {
        if let Some(key) = rl.get_key_pressed() {
            self.handle_keypress(key, rl, thread);
        }
        let mouse_pos = rl.get_mouse_position();
        let char_press = rl.get_char_pressed();
        let delet_press = rl.is_key_pressed(KeyboardKey::KEY_BACKSPACE);
        let mut d = rl.begin_drawing(thread);

        d.clear_background(Color::GRAY);

        d.draw_text("Press Q to quit", 12, 5, 20, Color::DARKGRAY);
        d.draw_text(
            "Press L to intizalizete the Game of Life",
            12,
            30,
            20,
            Color::WHITE,
        );
        let teste_rec = Rectangle::new(12.0, 100.0, 300.0, 25.0);
        d.gui_button(teste_rec, "AAAAA");
    }

    pub fn run(mut self: Self, rl: RaylibHandle, thread: RaylibThread) {
        game_loop::run(rl, thread, 60, move |rl, thread| {
            if rl.is_key_pressed(KeyboardKey::KEY_F1) {
                self.select_game = SelectedGame::None
            }

            match &mut self.select_game {
                SelectedGame::Life(life) => {
                    life.run(rl, thread);
                }
                SelectedGame::LifeSeter(setter) => {
                    setter.run(rl, thread);
                    if setter.is_setuped() {
                        self.init_life();
                    }
                }
                _ => self.run_hub(rl, thread),
            }
        });
    }
}

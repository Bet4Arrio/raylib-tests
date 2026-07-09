use crate::life::GameOfLife;
use sola_raylib::prelude::*;

enum SelectedGame {
    Life(GameOfLife),
    None,
}
pub struct GameHub {
    select_game: SelectedGame,
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
            KeyboardKey::KEY_ONE => {
                self.init_life();
            }
            _ => {}
        }
    }
    fn init_life(self: &mut Self) {
        self.select_game = SelectedGame::Life(GameOfLife::init(50, 60, 10, 10))
    }
    fn run_hub(self: &mut Self, rl: &mut RaylibHandle, thread: &RaylibThread) {
        if let Some(key) = rl.get_key_pressed() {
            self.handle_keypress(key, rl, thread);
        }
        let mut d = rl.begin_drawing(thread);
        d.clear_background(Color::GRAY);

        d.draw_text("Press Q to quit", 12, 5, 20, Color::DARKGRAY);
        d.draw_text(
            "Press 1 to intizalizete the Game of Life",
            12,
            30,
            20,
            Color::WHITE,
        );
    }
    pub fn run(mut self: Self, rl: RaylibHandle, thread: RaylibThread) {
        game_loop::run(rl, thread, 60, move |rl, thread| {
            match &mut self.select_game {
                SelectedGame::None => self.run_hub(rl, thread),
                SelectedGame::Life(life) => {
                    life.run(rl, thread);
                }
            }
            // Programmatic quit. ESC and the OS close button still work as the
            // sola_raylib defaults; this is how you'd wire a quit menu item, 1gamepad
            // button, or any other in-game exit path on native or web.
        });
    }
}

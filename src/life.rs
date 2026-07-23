use crate::core::traits::Component;
use crate::ui::button_input::ButtonInput;
use crate::ui::text_input::TextInput;
use sola_raylib::prelude::*;
use std::vec;

#[derive(Clone)]
pub struct Simulation {
    pub matrix: Vec<bool>,
    pub aux_vec: Vec<bool>,
    pub height: i32,
    pub width: i32,
}

impl Simulation {
    pub fn init(h: i32, w: i32) -> Self {
        Simulation {
            matrix: vec![false; (h * w) as usize],
            aux_vec: vec![false; (h * w) as usize],
            height: h,
            width: w,
        }
    }
    pub fn idx_from_cord(self: &Self, row: i32, col: i32) -> i32 {
        return (row * self.width) + col;
    }
    pub fn coord_from_idx(self: &Self, idx: i32) -> (i32, i32) {
        let row = idx / self.width;
        let col = idx % self.width;
        return (row, col);
    }
    pub fn rand_instance(self: &mut Self) {
        self.matrix = (0..self.area()).map(|_| rand::random_ratio(1, 3)).collect()
    }

    fn get_neighbors_idx(self: &Self, idx: i32) -> Vec<usize> {
        let cell = self.coord_from_idx(idx);
        let neighbors_coords: Vec<(i32, i32)> = vec![
            (cell.0 + 1, cell.1),     // direira
            (cell.0 - 1, cell.1),     // esquerda
            (cell.0, cell.1 + 1),     // baixo
            (cell.0, cell.1 - 1),     // cima
            (cell.0 + 1, cell.1 + 1), //
            (cell.0 - 1, cell.1 - 1), //
            (cell.0 + 1, cell.1 - 1), //
            (cell.0 - 1, cell.1 + 1), //
        ]
        .into_iter()
        .filter(|cord| cord.0 >= 0 && cord.0 < self.height && cord.1 >= 0 && cord.1 < self.width)
        .collect();

        neighbors_coords
            .iter()
            .map(|cell| self.idx_from_cord(cell.0, cell.1) as usize)
            .collect()
    }

    fn count_life_neighbors(self: &Self, idx: i32) -> i32 {
        self.get_neighbors_idx(idx)
            .iter()
            .fold(0, |acc, x| if self.matrix[*x] { acc + 1 } else { acc })
    }
    pub fn simulate(self: &mut Self) {
        // Subpopulação: Célula viva com < 2 vizinhos vivos morre.
        // Sobrevivência: Célula viva com 2 ou 3 vizinhos vivos continua viva.
        // Superpopulação: Célula viva com > 3 vizinhos vivos morre.
        // Reprodução: Célula morta com exatamente 3 vizinhos vivos ganha vida.
        //
        self.aux_vec = (0..self.area())
            .map(|i| {
                let vizinhos = self.count_life_neighbors(i);
                if self.matrix[i as usize] {
                    if vizinhos > 3 || vizinhos < 2 {
                        false
                    } else {
                        true
                    }
                } else {
                    if vizinhos == 3 { true } else { false }
                }
            })
            .collect();
        self.matrix = (0..self.area()).map(|i| self.aux_vec[i as usize]).collect()
    }
    pub fn area(self: &Self) -> i32 {
        return self.width * self.height;
    }
}

enum GameStatus {
    Idle,
    Run,
}

#[derive(Debug, Clone, Copy)]
pub struct Cell {
    idx: usize,
    rect: Rectangle,
}

pub struct GameOfLifeSeter {
    pub h: i32,
    pub w: i32,
    pub setuped: bool,
    pub h_input: TextInput<i32>,
    pub w_input: TextInput<i32>,
    pub start_button: ButtonInput,
}

impl Default for GameOfLifeSeter {
    fn default() -> Self {
        GameOfLifeSeter {
            h_input: TextInput::new(Rectangle::new(12.0, 50.0, 300.0, 25.0), 11),
            w_input: TextInput::new(Rectangle::new(350.0, 50.0, 300.0, 25.0), 11),
            start_button: ButtonInput::new(
                Rectangle::new(150.0, 100.0, 100.0, 25.0),
                "Iniciar jogo".to_string(),
            ),
            setuped: false,
            h: 11,
            w: 11,
        }
    }
}
impl GameOfLifeSeter {
    fn get_ready(self: &mut Self) {
        self.setuped = true
    }
    fn handle_keypress(self: &mut Self, button: KeyboardKey) {
        match button {
            KeyboardKey::KEY_L => {
                self.get_ready();
            }
            _ => {}
        }
    }

    pub fn run(self: &mut Self, rl: &mut RaylibHandle, thread: &RaylibThread) {
        if let Some(key) = rl.get_key_pressed() {
            self.handle_keypress(key);
        }
        self.h_input.update_value(rl);
        self.w_input.update_value(rl);
        self.start_button.check(rl);
        if self.start_button.is_clicked() {
            self.get_ready();
        }
        let mut d = rl.begin_drawing(thread);
        d.clear_background(Color::GRAY);
        self.h_input.draw(&mut d);
        self.w_input.draw(&mut d);
        self.start_button.draw(&mut d);
    }
    pub fn is_setuped(self: &mut Self) -> bool {
        self.setuped
    }
}

pub struct GameOfLife {
    status: GameStatus,
    speed: i32,
    width: i32,
    height: i32,
    simulation: Simulation,
    entities: Vec<Cell>,
    camera: Camera2D,
    last_updt: f64,
}

impl GameOfLife {
    pub fn init(width: i32, height: i32, mat_h: i32, mat_w: i32) -> Self {
        let padding: i32 = 100;
        let cell_size: i32 = 10;
        let cell_gap: i32 = 1;
        let simu = Simulation::init(mat_h, mat_w);
        let area = simu.area();
        let ent: Vec<Cell> = (0..area)
            .map(|i| {
                let (row, col) = simu.coord_from_idx(i);
                let posy = (cell_size + cell_gap) * row;
                let posx = (cell_size + cell_gap) * col;
                Cell {
                    idx: i as usize,
                    rect: Rectangle {
                        x: (posx + padding) as f32,
                        y: (posy + padding) as f32,
                        width: cell_size as f32,
                        height: cell_size as f32,
                    },
                }
            })
            .collect();
        let camera = Camera2D {
            target: Vector2 {
                x: width as f32 / 2.0,
                y: height as f32 / 2.0,
            }, // Centralizado no jogador
            offset: Vector2 {
                x: width as f32 / 2.0,
                y: height as f32 / 2.0,
            }, // Centralizado na tela (800x600)
            rotation: 0.0,
            zoom: 1.0,
        };
        GameOfLife {
            status: GameStatus::Idle,
            speed: 1,
            entities: ent,
            simulation: simu,
            last_updt: 0.0,
            camera,
            width,
            height,
        }
    }

    fn handle_mouse(self: &mut Self, rl: &mut RaylibHandle, mut _thread: &RaylibThread) {
        todo!()
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
            KeyboardKey::KEY_R => {
                self.simulation.rand_instance();
            }
            KeyboardKey::KEY_ENTER => {
                self.status = match self.status {
                    GameStatus::Idle => GameStatus::Run,
                    GameStatus::Run => GameStatus::Idle,
                };
            }
            KeyboardKey::KEY_SPACE => {
                self.speed = (self.speed + 1) % 11;
                if self.speed < 1 {
                    self.speed = 1;
                }
            }
            KeyboardKey::KEY_N => {
                self.simulation.simulate();
            }
            KeyboardKey::KEY_W => {
                if self.camera.offset.y < 0.0 {
                    self.camera.offset.y = 0.0;
                } else {
                    self.camera.offset += Vector2 { x: 0.0, y: -10.0 };
                }
            }
            KeyboardKey::KEY_S => {
                self.camera.offset += Vector2 { x: 0.0, y: 10.0 };
            }
            KeyboardKey::KEY_A => {
                if self.camera.offset.x < 0.0 {
                    self.camera.offset.x = 0.0;
                } else {
                    self.camera.offset += Vector2 { x: -10.0, y: 0.0 };
                }
            }
            KeyboardKey::KEY_D => {
                if self.camera.offset.x > 0.0 {
                    self.camera.offset.x = 0.0;
                } else {
                    self.camera.offset += Vector2 { x: 10.0, y: 0.0 };
                }
            }

            _ => {}
        }
    }
    pub fn run(self: &mut Self, rl: &mut RaylibHandle, thread: &RaylibThread) {
        if let Some(key) = rl.get_key_pressed() {
            self.handle_keypress(key, rl, thread);
        }
        let tim = rl.get_time();

        // self.handle_mouse(rl, thread);

        let text = format!(
            "Game of life spd {}/seq, {}x{}",
            self.speed, self.simulation.width, self.simulation.height
        );
        let mut d = rl.begin_drawing(thread);
        d.clear_background(Color::GRAY);

        {
            let mut mode2d = d.begin_mode2D(self.camera);
            // Desenhe seus objetos, tiles e jogador aqui
            // ex: mode2d.draw_circle_v(player_pos, 20.0, Color::RED);
            for cell in &self.entities {
                if self.simulation.matrix[cell.idx] {
                    mode2d.draw_rectangle_rec(cell.rect, Color::CYAN);
                } else {
                    mode2d.draw_rectangle_rec(cell.rect, Color::DARKCYAN);
                }
            }
        }

        d.draw_text("Press Q to quit", 12, 5, 20, Color::DARKGRAY);

        d.draw_text(&text, 12, 25, 20, Color::BLACK);

        match self.status {
            GameStatus::Idle => {
                d.draw_text("Press ENTER TO Run ", 200, 5, 20, Color::GREEN);
            }
            GameStatus::Run => {
                let delta = tim - self.last_updt;
                if delta > 1.0 / (self.speed as f64) {
                    self.last_updt = tim;
                    self.simulation.simulate();
                }
                d.draw_text("Press ENTER TO STOP ", 200, 5, 20, Color::RED);
            }
        }
    }
}

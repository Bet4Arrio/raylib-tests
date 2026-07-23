pub mod core;
pub mod game;
pub mod kdtree;
pub mod life;
pub mod ui;
fn main() {
    let (rl, thread) = sola_raylib::init()
        .size(640, 480)
        .title("Hello, sola_raylib")
        .highdpi()
        .resizable()
        .build();
    let game = game::GameHub::init(rl.get_screen_width(), rl.get_screen_height());
    game.run(rl, thread);
}

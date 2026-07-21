use sola_raylib::prelude::*;

pub trait Component {
    fn draw(&self, d: &mut RaylibDrawHandle);
    fn check(&mut self, rl: &mut RaylibHandle);
}

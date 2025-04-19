const WIDTH: usize = 64;
const HEIGHT: usize = 32;

mod chip8;
mod display;
use chip8::Chip8;
use minifb::{Window, WindowOptions, Key};

fn main() {
    let mut chip8: Chip8 = Chip8::default();
    chip8.run();
}

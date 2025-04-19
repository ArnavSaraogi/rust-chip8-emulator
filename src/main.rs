mod chip8;
mod display;
use chip8::Chip8;

fn main() {
    let mut chip8: Chip8 = Chip8::default();
    chip8.run();
}

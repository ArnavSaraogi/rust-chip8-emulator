mod chip8;
mod display;
mod timers;
use chip8::Chip8;

fn main() {
    let mut chip8: Chip8 = Chip8::default();
    chip8.run();
}

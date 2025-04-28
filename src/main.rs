mod chip8;
mod display;
mod timers;
use chip8::Chip8;

fn main() {
    let mut chip8: Chip8 = Chip8::default();
    let path = "roms/pong.rom";
    let result = chip8.load_rom(path);
    if result.is_err() {
        println!("Error in reading file");
    } else {
        println!("ROM loaded");
    }
    chip8.run();
}

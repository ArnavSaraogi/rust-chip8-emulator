use crate::display::Display;
use crate::timers::Timers;
use std::time::{Duration, Instant};
use std::thread::sleep;

const NUM_ADRESSES: usize = 4096;
const STACK_MAX: usize = 16;
const NUM_REGISTERS: usize = 16;
const TICK_RATE: f64 = 1.0 / 60.0;
const INSTRUCTIONS_PER_FRAME: usize = 700 / 60;

#[derive(Debug)]
pub struct Chip8 {
    memory: [u8; NUM_ADRESSES],
    display: Display,
    program_counter: u16,
    i_register: u16,
    stack: [u16; STACK_MAX],
    timers: Timers,
    variable_registers: [u8; NUM_REGISTERS],
}

impl Chip8 {
    pub fn default() -> Self {
        let mut chip8 = Chip8 {
            memory: [0; NUM_ADRESSES],
            display: Display::default(),
            program_counter: 0,
            i_register: 0,
            stack: [0; STACK_MAX],
            timers: Timers::default(),
            variable_registers: [0; NUM_REGISTERS],
        };
        
        chip8.load_fonts();

        chip8
    }

    pub fn run(&mut self) {
        let mut last_tick = Instant::now();

        while self.display.is_open() && !self.display.is_key_down(minifb::Key::Escape) {
            // 1. fetch opcode
            

            // 2. decode opcode


            // 3. execute it (update state: memory, registers, display, sound, etc.)


            // 4. update timers
            self.timers.decrement_timers();

            // 5. render display
            self.display.render();

            // 6. handle inputs

            //ensure while loop runs at 60 hz
            let time_elapsed = last_tick.elapsed();
            if time_elapsed < Duration::from_secs_f64(TICK_RATE) {
                sleep(Duration::from_secs_f64(TICK_RATE) - time_elapsed);
            }
            last_tick = Instant::now();
        }
    }

    fn load_fonts(&mut self) {
        let fonts: [u8; 80] = [0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
                                0x20, 0x60, 0x20, 0x20, 0x70, // 1
                                0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
                                0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
                                0x90, 0x90, 0xF0, 0x10, 0x10, // 4
                                0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
                                0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
                                0xF0, 0x10, 0x20, 0x40, 0x40, // 7
                                0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
                                0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
                                0xF0, 0x90, 0xF0, 0x90, 0x90, // A
                                0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
                                0xF0, 0x80, 0x80, 0x80, 0xF0, // C
                                0xE0, 0x90, 0x90, 0x90, 0xE0, // D
                                0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
                                0xF0, 0x80, 0xF0, 0x80, 0x80]; // F
        
        self.memory[0x050..0x0A0].copy_from_slice(&fonts);
    }

    pub fn print(&self) {
        print!("{:#?}", self);
    }
}

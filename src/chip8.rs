use crate::display::Display;
use crate::timers::Timers;
use std::fs::File;
use std::io::{self, Read};
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
    pc: u16,
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
            pc: 0x200,
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
            for _ in 0..INSTRUCTIONS_PER_FRAME {
                //fetch, decode, execute opcode (update state: memory, registers, display, sound, etc.)
                self.execute_opcode();
            }

            // 4. update timers
            self.timers.decrement_timers();

            // 5. render display
            self.display.render();

            // 6. handle inputs

            //ensure while loop runs at 60 hz
            let time_elapsed = last_tick.elapsed();
            let target_duration = Duration::from_secs_f64(TICK_RATE);
            if time_elapsed < target_duration {
                sleep(target_duration - time_elapsed);
            }
            last_tick = Instant::now();
        }
    }

    pub fn load_rom(&mut self, path: &str) -> io::Result<()>{
        let mut file = File::open(path)?;
        file.read(&mut self.memory[0x200..])?;
        Ok(())
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

    fn execute_opcode(&mut self) {
        let opcode = self.bytes_to_opcode();
        //match statment
    }

    fn bytes_to_opcode(&mut self) -> u16 {
        let high_byte = self.memory[self.pc as usize];
        let low_byte = self.memory[(self.pc + 1) as usize];
        ((high_byte as u16) << 8) | (low_byte as u16)
    }

    pub fn print(&self) {
        print!("{:#?}", self);
    }
}

use crate::display::Display;
use crate::timers::Timers;
use std::fs::File;
use std::io::{self, Read};
use std::time::{Duration, Instant};
use std::thread::sleep;
use rand::Rng;

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
                let opcode = self.fetch_opcode();
                self.execute_opcode(opcode);
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

    fn execute_opcode(&mut self, opcode: u16) {
        let vx = (opcode & 0x0F00) >> 8;
        let vy = (opcode & 0x00F0) >> 4;
        let n = opcode & 0x000F;
        let nn = (opcode & 0x00FF) as u8;
        let address = opcode & 0x0FFF;
        
        match opcode & 0xF000 {
            0x0000 => {
                //0NNN --> call machine code at address NNN
                //00E0 --> clear display
                //00EE --> Return from subroutine
            }
            0x1000 => self.pc = address,  // set pc to NNN
            0x2000 => {}  // NEED TO IMPLEMENT, call subroutine at NNN 
            0x3000 => if self.variable_registers[vx as usize] == nn {self.pc += 2}, // skips next instruction if VX == NN
            0x4000 => if self.variable_registers[vx as usize] != nn {self.pc += 2}, // skips next instruction if VX != NN
            0x5000 => if self.variable_registers[vx as usize] == self.variable_registers[vy as usize] {self.pc += 2}, // skips next instruction if VX == VY
            0x6000 => self.variable_registers[vx as usize] = nn, // sets VX to NN
            0x7000 => self.variable_registers[vx as usize] = self.variable_registers[vx as usize].wrapping_add(nn), // adds NN to VX
            0x8000 => {
                match n {
                    0x0000 => self.variable_registers[vx as usize] = self.variable_registers[vy as usize], // sets VX to value of VY
                    0x0001 => self.variable_registers[vx as usize] |= self.variable_registers[vy as usize], // sets VX to VX OR VY
                    0x0002 => self.variable_registers[vx as usize] &= self.variable_registers[vy as usize], // sets VX to VX AND VY
                    0x0003 => self.variable_registers[vx as usize] ^= self.variable_registers[vy as usize], // sets VX to VX XOR VY
                    0x0004 => { // adds VY to VX. Makes VF 1 or 0 based on if it overflows or doesn't, respectively
                        if (self.variable_registers[vx as usize] as u16) + (self.variable_registers[vy as usize] as u16) > 255 {
                            self.variable_registers[15] = 1;
                        } else {
                            self.variable_registers[15] = 0;
                        }
                        self.variable_registers[vx as usize] = self.variable_registers[vx as usize].wrapping_add(self.variable_registers[vy as usize]);
                    } 
                    0x0005 => { // subtracts VY from VX. Makes VF 0 or 1 based on if it underflows or doesn't, respectively
                        if self.variable_registers[vx as usize] < self.variable_registers[vy as usize] {
                            self.variable_registers[15] = 0;
                        } else {
                            self.variable_registers[15] = 1;
                        }
                        self.variable_registers[vx as usize] = self.variable_registers[vx as usize].wrapping_sub(self.variable_registers[vy as usize]);
                    }
                    0x0006 => { // shifts VX right, stores least significant bit in VF
                        let lsb = self.variable_registers[vx as usize] & 1;
                        self.variable_registers[vx as usize] = self.variable_registers[vx as usize] >> 1;
                        self.variable_registers[15] = lsb;
                    }
                    0x0007 => { // sets VX to VY - VX. Makes VF 0 or 1 based on if it underflows or doesn't, respectively
                        if self.variable_registers[vy as usize] < self.variable_registers[vx as usize] {
                            self.variable_registers[15] = 0;
                        } else {
                            self.variable_registers[15] = 1;
                        }
                        self.variable_registers[vx as usize] = self.variable_registers[vy as usize].wrapping_sub(self.variable_registers[vx as usize]);
                    }
                    0x000E => { // shifts VX to left, stores most significant bit in VF
                        let msb = self.variable_registers[vx as usize] >> 7;
                        self.variable_registers[vx as usize] = self.variable_registers[vx as usize] << 1;
                        self.variable_registers[15] = msb;
                    }
                    _ => ()
                }
            }
            0xA000 => self.i_register = address, // sets index register to address
            0xB000 => self.pc = address + (self.variable_registers[0] as u16), //jumps to address NNN + V0
            0xC000 => {
                let mut rng = rand::rng();
                let random_num: u8 = rng.random();
                self.variable_registers[vx as usize] = nn & random_num;
            }
            0xD000 => { //drawing sprite on display
                //NEED TO IMPLEMENT
            }
            0xE000 => {
                match opcode & 0x0FF {
                    0x009E => { //if key in VX (lowest nibble) currently held down, skip next instruction
                        //NEED TO IMPLEMENT
                    }
                    0x00A1 => { //if key in VX (lowest nibble) not held down, skip next instruction
                        //NEED TO IMPLEMENT
                    }
                    _ => {}
                }
            }
            0xF000 => {
                match opcode & 0x00FF {
                    0x0007 => self.variable_registers[vx as usize] = self.timers.dt_register, // sets VX to value of delay timer
                    0x000A => { // key press awaited then stored in VX
                        //NEED TO IMPLEMENT
                    }
                    0x0015 => self.timers.dt_register = self.variable_registers[vx as usize], // sets delay timer to VX
                    0x0018 => self.timers.st_register = self.variable_registers[vx as usize], // sets sound timer to VX
                    0x001E => self.i_register = self.i_register.wrapping_add(self.variable_registers[vx as usize] as u16), // adds VX to I
                    0x0029 => self.point_i_to_character(vx), // index register set to address of character in VX
                    0x0033 => { // stores digits of decimal conversion of value in VX in i, i + 1, i + 2 in mem, 
                        let mut num = self.variable_registers[vx as usize];
                        for i in (0..=2).rev() {
                            self.memory[self.i_register as usize + i] = num % 10;
                            num /= 10;
                        }
                    }
                    0x0055 => { // stores registers into memory up till VX
                        for i in 0..=vx {
                            self.memory[(self.i_register + i) as usize] = self.variable_registers[i as usize];
                        }
                    }
                    0x0065 => { // loads registers from memory up till VX
                        for i in 0..=vx {
                            self.variable_registers[i as usize] = self.memory[(self.i_register + i) as usize];
                        }   
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }

    fn fetch_opcode(&mut self) -> u16 {
        let high_byte = self.memory[self.pc as usize];
        let low_byte = self.memory[(self.pc + 1) as usize];
        self.pc += 2;
        ((high_byte as u16) << 8) | (low_byte as u16)
    }

    fn point_i_to_character(&mut self, vx: u16) {
        match self.variable_registers[vx as usize] & 0x000F {
            0x0000 => self.i_register = 0x050,
            0x0001 => self.i_register = 0x055,
            0x0002 => self.i_register = 0x05A,
            0x0003 => self.i_register = 0x05F,
            0x0004 => self.i_register = 0x064,
            0x0005 => self.i_register = 0x069,
            0x0006 => self.i_register = 0x06E,
            0x0007 => self.i_register = 0x073,
            0x0008 => self.i_register = 0x078,
            0x0009 => self.i_register = 0x07D,
            0x000A => self.i_register = 0x082,
            0x000B => self.i_register = 0x087,
            0x000C => self.i_register = 0x08C,
            0x000D => self.i_register = 0x091,
            0x000E => self.i_register = 0x096,
            0x000F => self.i_register = 0x09B,
            _ => {}
        }
    }

    pub fn print(&self) {
        print!("{:#?}", self);
    }
}

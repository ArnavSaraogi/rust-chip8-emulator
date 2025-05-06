use crate::display::Display;
use crate::timers::Timers;
use crate::stack::Stack;
use std::fs::File;
use std::io::{self, Read};
use std::time::{Duration, Instant};
use std::thread::sleep;
use rand::Rng;
use minifb::Key;


const NUM_ADRESSES: usize = 4096;
const NUM_REGISTERS: usize = 16;
const TICK_RATE: f64 = 1.0 / 60.0;
const INSTRUCTIONS_PER_FRAME: usize = 600 / 60;
const NUM_KEYS: usize = 16;

#[derive(Debug)]
pub struct Chip8 {
    memory: [u8; NUM_ADRESSES],
    display: Display,
    pc: u16,
    i_register: u16,
    stack: Stack,
    timers: Timers,
    variable_registers: [u8; NUM_REGISTERS],
    key_states: [bool; NUM_KEYS],
}

impl Chip8 {
    pub fn default() -> Self {
        let mut chip8 = Chip8 {
            memory: [0; NUM_ADRESSES],
            display: Display::default(),
            pc: 0x200,
            i_register: 0,
            stack: Stack::default(),
            timers: Timers::default(),
            variable_registers: [0; NUM_REGISTERS],
            key_states: [false; NUM_KEYS]
        };
        
        chip8.load_fonts();

        chip8
    }

    pub fn run(&mut self) {
        let mut last_tick = Instant::now();

        while self.display.is_open() && !self.display.is_key_down(minifb::Key::Escape) {            
            for _ in 0..INSTRUCTIONS_PER_FRAME {
                let opcode = self.fetch_opcode();
                self.execute_opcode(opcode);
            }
            self.update_keys();
            self.timers.decrement_timers();
            self.display.render();

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
                match opcode {
                    0x00E0 => self.display.clear(), // clear display
                    0x00EE => self.pc = self.stack.pop(), // return from subroutine
                    _ => {},
                }
            }
            0x1000 => self.pc = address,  // set pc to NNN
            0x2000 => { // call subroutine at NNN 
                self.stack.push(self.pc);
                self.pc = address;
            }
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
                let x_cord = self.variable_registers[vx as usize] % 64;
                let y_cord = self.variable_registers[vy as usize] % 32;
                self.variable_registers[15] = 0;
                self.draw_sprite_to_display(n, x_cord, y_cord);
            }
            0xE000 => {
                let key = self.variable_registers[vx as usize];
                let key_pressed = self.key_states[key as usize];
                match opcode & 0x00FF {
                    0x009E => if key_pressed {self.pc += 2}, //if key in VX (lowest nibble) currently held down, skip next instruction
                    0x00A1 => if !key_pressed {self.pc += 2} //if key in VX (lowest nibble) not held down, skip next instruction
                    _ => {}
                }
            }
            0xF000 => {
                match opcode & 0x00FF {
                    0x0007 => self.variable_registers[vx as usize] = self.timers.dt_register, // sets VX to value of delay timer
                    0x000A => { // key press awaited then stored in VX
                        let mut key_pressed = false;
                        let mut key = 0;
                        for i in 0..NUM_KEYS {
                            if self.key_states[i] {
                                key_pressed = true;
                                key = i;
                                break;
                            }
                        }
                        if key_pressed {
                            self.variable_registers[vx as usize] = key as u8;
                        } else {
                            self.pc -= 2;
                        }
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

    fn draw_sprite_to_display(&mut self, n: u16, x_cord: u8, y_cord: u8) {
        let mut x = x_cord as usize;
        let mut y = y_cord as usize;

        for row in 0..n {
            if y >= 32 {
                break;
            }
            let sprite_byte = self.memory[(self.i_register + row) as usize];
            for i in 0..8 {
                if x >= 64 {
                    break;
                }
                let pixel_bit = (sprite_byte >> (7 - i)) & 1;
                if pixel_bit == 1 && self.display.frame_buffer[y][x] {
                    self.display.frame_buffer[y][x] = false;
                    self.variable_registers[15] = 1;
                } else if pixel_bit == 1 && !(self.display.frame_buffer[y][x]) {
                    self.display.frame_buffer[y][x] = true;
                }
                x += 1;
            }
            x = x_cord as usize;
            y += 1;
        }
    }

    fn update_keys(&mut self) {
        self.key_states = [false; 16];
        let keys = self.display.window.get_keys();
        for key in keys {
            match key {
                Key::Key1 => self.key_states[0x1] = true,
                Key::Key2 => self.key_states[0x2] = true,
                Key::Key3 => self.key_states[0x3] = true,
                Key::Key4 => self.key_states[0xC] = true,
                Key::Q    => self.key_states[0x4] = true,
                Key::W    => self.key_states[0x5] = true,
                Key::E    => self.key_states[0x6] = true,
                Key::R    => self.key_states[0xD] = true,
                Key::A    => self.key_states[0x7] = true,
                Key::S    => self.key_states[0x8] = true,
                Key::D    => self.key_states[0x9] = true,
                Key::F    => self.key_states[0xE] = true,
                Key::Z    => self.key_states[0xA] = true,
                Key::X    => self.key_states[0x0] = true,
                Key::C    => self.key_states[0xB] = true,
                Key::V    => self.key_states[0xF] = true,
                _ => {}
            }
        }
    }
}

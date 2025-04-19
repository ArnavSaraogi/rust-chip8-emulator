use crate::display::{self, Display};

const NUM_ADRESSES: usize = 4096;
const DISPLAY_WIDTH: usize = 64;
const DISPLAY_HEGHT: usize = 32; 
const STACK_MAX: usize = 16;
const NUM_REGISTERS: usize = 16;

#[derive(Debug)]
pub struct Chip8 {
    memory: [u8; NUM_ADRESSES],
    display: Display,
    program_counter: u16,
    i_register: u16,
    stack: [u16; STACK_MAX],
    dt_register: u8,
    st_register: u8,
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
            dt_register: 0,
            st_register: 0,
            variable_registers: [0; NUM_REGISTERS],
        };
        
        chip8.load_fonts();

        chip8
    }

    pub fn run(&mut self) {
        self.display.render();
    }

    pub fn print(&self) {
        print!("{:#?}", self);
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
}

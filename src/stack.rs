const STACK_MAX: usize = 16;

#[derive(Debug)]
pub struct Stack {
    pub stack: [u16; STACK_MAX],
    pub stack_pointer: usize
}

impl Stack {
    pub fn default() -> Self {
        let stack = Stack {
            stack: [0; STACK_MAX],
            stack_pointer: 0,
        };

        stack
    }

    pub fn push(&mut self, address: u16) {
        if self.stack_pointer >= STACK_MAX {
            panic!("Stack overflow");
        }

        self.stack[self.stack_pointer] = address;
        self.stack_pointer += 1;
    }

    pub fn pop(&mut self) -> u16 {
        if self.stack_pointer <= 0 {
            panic!("Stack underflow");
        }

        self.stack_pointer -= 1;
        self.stack[self.stack_pointer]
    }
}
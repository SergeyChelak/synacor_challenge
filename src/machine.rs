// const MAX_MEMORY_SIZE: usize = 1 << 16;
const REGISTERS_COUNT: usize = 8;

pub struct Machine {
    memory: Vec<u8>,
    register: [u16; REGISTERS_COUNT],
    stack: Vec<u16>,
}

impl Machine {
    pub fn new(program: Vec<u8>) -> Self {
        Machine { 
            memory: program, 
            register: [0; REGISTERS_COUNT], 
            stack: Vec::new()
        }
    }

    pub fn run(&mut self) {
        // loop {
            
        // }
    }
}
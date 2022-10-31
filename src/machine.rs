const REGISTERS_COUNT: usize = 8;

pub struct Machine {
    memory: Vec<u8>,
    register: [u16; REGISTERS_COUNT],
    stack: Vec<u16>,

    cp: usize,      // code pointer

    is_running: bool,
}

impl Machine {
    pub fn new(program: Vec<u8>) -> Self {
        Machine { 
            memory: program, 
            register: [0; REGISTERS_COUNT], 
            stack: Vec::new(),
            cp: 0,

            is_running: false,
        }
    }

    // -- main loop
    pub fn run(&mut self) {
        self.is_running = true;
        while self.is_running {
            let instruction = self.next();
            match instruction {
                0 => 
                    self.halt(),
                19 =>
                    self.out(),
                21 =>
                    self.noop(),
                _ =>
                    panic!("Unhandled instruction {}", instruction),
            }
        }
    }

    fn next(&mut self) -> u16 {
        let value = self.read_memory();
        self.cp += 1;
        value
    }

    fn read_memory(&self) -> u16 {
        let pos = self.cp << 1;
        self.memory[pos] as u16 | (self.memory[pos + 1] as u16) << 8
    }

    // -- operations 
    // 0: stop execution and terminate the program
    fn halt(&mut self) {
        self.is_running = false;
    }

    // 19: write the character represented by ascii code <a> to the terminal
    fn out(&mut self) {
        let arg = self.next() as u8 as char;
        print!("{}", arg);
    }

    // 21: no operation
    fn noop(&self) {
        // no op
    }

    // -- utils
}
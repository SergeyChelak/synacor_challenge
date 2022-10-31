const REGISTERS_COUNT: usize = 8;
const REGISTERS_OFFSET: usize = 32768;

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
            let instruction = self.read_next();
            match instruction {
                 0 => self.halt(),
                 6 => self.jmp(),
                 7 => self.jt(),
                 8 => self.jf(),
                19 => self.out(),
                21 => self.noop(),
                _ =>
                    panic!("Unhandled instruction {}", instruction),
            }
        }
    }

    fn read_next(&mut self) -> u16 {
        let value = self.read_memory();
        self.cp += 1;
        value
    }

    fn read_next_as_arg(&mut self) -> u16 {
        let value = self.read_next();
        if value < REGISTERS_OFFSET as u16 {
            value
        } else {
            let register_idx = value as usize - REGISTERS_OFFSET;
            self.register[register_idx]
        }
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

    // 6: jump to <a>
    fn jmp(&mut self) {
        let jmp_addr = self.read_next();
        self.cp = jmp_addr as usize;
    }

    // 7: if <a> is nonzero, jump to <b>
    fn jt(&mut self) {
        let a = self.read_next_as_arg();
        let b = self.read_next();
        if a != 0 {
            self.cp = b as usize;
        }
    }

    // 8: if <a> is zero, jump to <b>
    fn jf(&mut self) {
        let a = self.read_next_as_arg();
        let b = self.read_next();
        if a == 0 {
            self.cp = b as usize;
        }
    }

    // 19: write the character represented by ascii code <a> to the terminal
    fn out(&mut self) {
        let arg = self.read_next_as_arg() as u8 as char;
        print!("{}", arg);
    }

    // 21: no operation
    fn noop(&self) {
        // no op
    }

    // -- utils
}
use std::io;

const REGISTERS_COUNT: usize = 8;
const REGISTERS_OFFSET: usize = 32768;
const MEMORY_SIZE: usize = 1 << 15; // words

pub struct Machine {
    memory: [u16; MEMORY_SIZE],
    register: [u16; REGISTERS_COUNT],
    stack: Vec<u16>,
    cp: usize,      // code pointer
    input_buffer: Vec<u8>,
    is_running: bool,
}

impl Machine {
    pub fn new(program: Vec<u8>, script: Vec<String>) -> Self {
        assert_eq!(program.len() % 2, 0, "Incorrect binary size");
        assert!(program.len() / 2 <= MEMORY_SIZE, "Binary is too big to fit the memory size");
        let mut memory: [u16; MEMORY_SIZE] = [0; MEMORY_SIZE];
        for i in (0..program.len()).step_by(2) {
            memory[i >> 1] = u16::from_le_bytes([program[i], program[i + 1]]);
        }        
        
        let mut input_buffer: Vec<u8> = Vec::new();
        for str in script.iter().rev() {            
            input_buffer.push('\n' as u8);
            for byte in str.as_bytes().iter().rev() {
                input_buffer.push(*byte);
            }            
        }

        Machine { 
            memory, 
            register: [0; REGISTERS_COUNT], 
            stack: Vec::new(),
            cp: 0,
            input_buffer,
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
                 1 => self.set(),
                 2 => self.push(),
                 3 => self.pop(),
                 4 => self.eq(),
                 5 => self.gt(),
                 6 => self.jmp(),
                 7 => self.jt(),
                 8 => self.jf(),
                 9 => self.add(),
                10 => self.mult(),
                11 => self.mod_op(),
                12 => self.and(),
                13 => self.or(),
                14 => self.not(),
                15 => self.rmem(),
                16 => self.wmem(),
                17 => self.call(),
                18 => self.ret(),
                19 => self.out(),
                20 => self.in_op(),
                21 => self.noop(),
                _ =>
                    panic!("Unhandled instruction {}", instruction),
            }
        }
    }

    #[inline]
    fn read_next(&mut self) -> u16 {
        let value = self.read_memory_at(self.cp);
        self.cp += 1;
        value
    }

    #[inline]
    fn read_value(&mut self) -> u16 {
        let value = self.read_next();
        if value < REGISTERS_OFFSET as u16 {
            value
        } else {
            let register_idx = value as usize - REGISTERS_OFFSET;
            self.register[register_idx]
        }
    }

    #[inline]
    fn read_register_idx(&mut self) -> usize {        
        let value = self.read_next() as usize;
        assert!(value >= REGISTERS_OFFSET, "Register index access violation");
        value - REGISTERS_OFFSET
    }

    #[inline]
    fn read_memory_at(&self, address: usize) -> u16 {
        assert!(address < REGISTERS_OFFSET, "Read memory violation");
        self.memory[address]
    }

    #[inline]
    fn write_memory_at(&mut self, address: usize, value: u16) {
        assert!(address < REGISTERS_OFFSET, "Write memory violation");
        self.memory[address] = value;
    }

    // -- operations 
    // 0: stop execution and terminate the program
    fn halt(&mut self) {
        self.is_running = false;
    }

    // 1:  set register <a> to the value of <b>
    fn set(&mut self) {
        let a = self.read_register_idx();
        let b = self.read_value();
        self.register[a] = b;
    }

    // 2: push <a> onto the stack
    fn push(&mut self) {
        let a = self.read_value();
        self.stack.push(a);
    }

    // 3: remove the top element from the stack and write it into <a>; empty stack = error
    fn pop(&mut self) {
        let a = self.read_register_idx();
        self.register[a] = self.stack.pop().unwrap();        
    }

    // 4: set <a> to 1 if <b> is equal to <c>; set it to 0 otherwise
    fn eq(&mut self) {
        let a = self.read_register_idx();
        let b = self.read_value();
        let c = self.read_value();
        self.register[a] = if b == c { 1 } else { 0 }
    }

    // 5: set <a> to 1 if <b> is greater than <c>; set it to 0 otherwise
    fn gt(&mut self) {
        let a = self.read_register_idx();
        let b = self.read_value();
        let c = self.read_value();
        self.register[a] = if b > c { 1 } else { 0 }
    }

    // 6: jump to <a>
    fn jmp(&mut self) {
        let jmp_addr = self.read_next();
        self.cp = jmp_addr as usize;
    }

    // 7: if <a> is nonzero, jump to <b>
    fn jt(&mut self) {
        let a = self.read_value();
        let b = self.read_next();
        if a != 0 {
            self.cp = b as usize;
        }
    }

    // 8: if <a> is zero, jump to <b>
    fn jf(&mut self) {
        let a = self.read_value();
        let b = self.read_next();
        if a == 0 {
            self.cp = b as usize;
        }
    }

    // 9: assign into <a> the sum of <b> and <c> (modulo 32768)
    fn add(&mut self) {
        let a = self.read_register_idx();
        let b = self.read_value();
        let c = self.read_value();
        self.register[a] = (b + c) % REGISTERS_OFFSET as u16;
    }

    // 10: store into <a> the product of <b> and <c> (modulo 32768)
    fn mult(&mut self) {
        let a = self.read_register_idx();
        let b = self.read_value() as usize;
        let c = self.read_value() as usize;
        self.register[a] = (b * c % REGISTERS_OFFSET) as u16;
    }

    // 11 store into <a> the remainder of <b> divided by <c>
    fn mod_op(&mut self) {
        let a = self.read_register_idx();
        let b = self.read_value();
        let c = self.read_value();
        self.register[a] = b % c 
    }

    // 12: stores into <a> the bitwise and of <b> and <c>
    fn and(&mut self) {
        let a = self.read_register_idx();
        let b = self.read_value();
        let c = self.read_value();
        self.register[a] = b & c;
    }

    // 13: stores into <a> the bitwise or of <b> and <c>
    fn or(&mut self) {
        let a = self.read_register_idx();
        let b = self.read_value();
        let c = self.read_value();
        self.register[a] = b | c;
    }

    // 14: stores 15-bit bitwise inverse of <b> in <a>
    fn not(&mut self) {
        let a = self.read_register_idx();
        let b = !self.read_value() & 0x7fff;
        self.register[a] = b
    }

    // 15: read memory at address <b> and write it to <a>
    fn rmem(&mut self) {
        let a = self.read_register_idx();
        let b = self.read_value();
        self.register[a] = self.read_memory_at(b as usize)
    }

    // 16: write the value from <b> into memory at address <a>
    fn wmem(&mut self) {
        let a = self.read_value() as usize; // addr
        let b = self.read_value();
        self.write_memory_at(a, b);
    }

    // 17: write the address of the next instruction to the stack and jump to <a>
    fn call(&mut self) {
        let jmp_addr = self.read_value();
        self.stack.push(self.cp as u16);
        self.cp = jmp_addr as usize;
    }

    // 18: remove the top element from the stack and jump to it; empty stack = halt
    fn ret(&mut self) {
        let jmp_addr = self.stack.pop().unwrap();
        self.cp = jmp_addr as usize;
    }
    
    // 19: write the character represented by ascii code <a> to the terminal
    fn out(&mut self) {
        let arg = self.read_value() as u8 as char;
        print!("{}", arg);
    }

    // 20: read a character from the terminal and write its ascii code to <a>
    // It can be assumed that once input starts, it will continue until a newline is encountered
    // This means that you can safely read whole lines from the keyboard and trust that they will be fully read
    fn in_op(&mut self) {        
        if self.input_buffer.is_empty() {
            let mut buffer = String::new();
            io::stdin().read_line(&mut buffer).unwrap();
            for byte in buffer.as_bytes().iter().rev() {
                self.input_buffer.push(*byte);
            }
        }
        let ascii = self.input_buffer.pop().unwrap() as u16;
        let a = self.read_register_idx();
        self.register[a] = ascii;
    }

    // 21: no operation
    fn noop(&self) {
        // no op
    }
}
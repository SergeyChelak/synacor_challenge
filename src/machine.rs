use std::io;

const REGISTERS_COUNT: usize = 8;
const REGISTERS_OFFSET: usize = 32768;
const MEMORY_SIZE: usize = 1 << 15; // words

#[derive(Debug)]
pub enum MachineError {
    UnexpectedOpcode(u16),
    MemoryAccessViolation(usize),   // address
    RegisterAccessViolation(usize), // address
    InvalidNumber(u16),             // number greater 32775 are invalid
    PopOnEmptyStack,
    EmptyInputBuffer,
    InputBufferError(io::Error),
    NotEnoughMemory(usize),         // program size
    InvalidProgramSize(usize),
}

pub struct Machine {
    memory: [u16; MEMORY_SIZE],
    register: [u16; REGISTERS_COUNT],
    stack: Vec<u16>,
    cp: usize, // code pointer
    input_buffer: Vec<u8>,
    is_running: bool,
}

impl Machine {
    pub fn new(program: &Vec<u8>) -> Result<Self, MachineError> {
        Ok(Machine {
            memory: Self::setup_memory(program)?,
            register: [0; REGISTERS_COUNT],
            stack: Vec::new(),
            cp: 0,
            input_buffer: Vec::new(),
            is_running: false,
        })
    }

    pub fn setup_memory(program: &Vec<u8>) -> Result<[u16; MEMORY_SIZE], MachineError> {
        let len = program.len();
        if program.len() % 2 != 0 {
            return Err(MachineError::InvalidProgramSize(len));
        }
        if program.len() / 2 > MEMORY_SIZE {
            return Err(MachineError::NotEnoughMemory(len));
        }
        let mut memory: [u16; MEMORY_SIZE] = [0; MEMORY_SIZE];
        for i in (0..len).step_by(2) {
            memory[i >> 1] = u16::from_le_bytes([program[i], program[i + 1]]);
        }
        Ok(memory)
    }

    pub fn write_to_input_buffer(&mut self, strings: &Vec<String>) {
        let mut input_buffer: Vec<u8> = Vec::new();
        for str in strings.iter().rev() {
            input_buffer.push('\n' as u8);
            for byte in str.as_bytes().iter().rev() {
                input_buffer.push(*byte);
            }
        }
        self.input_buffer = input_buffer;
    }

    // -- main loop
    pub fn run(&mut self) {
        self.is_running = true;
        while self.is_running {
            if let Err(error) = self.perform_operation() {
                self.is_running = false;
                println!("-- Machine terminated with error {:?}", error);
            }
        }
    }

    fn perform_operation(&mut self) -> Result<(), MachineError> {
        let operation = self.read_next()?;
        match operation {
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
            _ => Err(MachineError::UnexpectedOpcode(operation)),
        }
    }

    #[inline]
    fn read_next(&mut self) -> Result<u16, MachineError> {
        let value = self.read_memory_at(self.cp)?;
        self.cp += 1;
        Ok(value)
    }

    #[inline]
    fn read_value(&mut self) -> Result<u16, MachineError> {
        let value = self.read_next()?;
        if value < REGISTERS_OFFSET as u16 {
            if value > 32775 {
                Err(MachineError::InvalidNumber(value))
            } else {
                Ok(value)
            }
        } else {
            let register_idx = value as usize - REGISTERS_OFFSET;
            Ok(self.register[register_idx])
        }
    }

    #[inline]
    fn read_register_idx(&mut self) -> Result<usize, MachineError> {
        let value = self.read_next()? as usize;
        if value >= REGISTERS_OFFSET {
            Ok(value - REGISTERS_OFFSET)
        } else {
            Err(MachineError::RegisterAccessViolation(value))
        }
    }

    #[inline]
    fn write_register(&mut self, reg_idx: usize, value: u16) {
        self.register[reg_idx] = value;
    }

    #[inline]
    fn read_memory_at(&self, address: usize) -> Result<u16, MachineError> {
        if address < REGISTERS_OFFSET {
            Ok(self.memory[address])
        } else {
            Err(MachineError::MemoryAccessViolation(address))
        }
    }

    #[inline]
    fn write_memory_at(&mut self, address: usize, value: u16) -> Result<(), MachineError> {
        if address < REGISTERS_OFFSET {
            self.memory[address] = value;
            Ok(())
        } else {
            Err(MachineError::MemoryAccessViolation(address))
        }
    }

    #[inline]
    fn read_register_idx_unary_arg(&mut self) -> Result<(usize, u16), MachineError> {
        let a = self.read_register_idx()?;
        let b = self.read_value()?;
        Ok((a, b))
    }

    #[inline]
    fn read_register_idx_binary_args(&mut self) -> Result<(usize, u16, u16), MachineError> {
        let (a, b) = self.read_register_idx_unary_arg()?;
        let c = self.read_value()?;
        Ok((a, b, c))
    }

    // -- operations
    // halt: (0) -- stop execution and terminate the program
    fn halt(&mut self) -> Result<(), MachineError> {
        self.is_running = false;
        Ok(())
    }

    // set: (1 a b) -- set register <a> to the value of <b>
    fn set(&mut self) -> Result<(), MachineError> {
        let (a, b) = self.read_register_idx_unary_arg()?;
        self.write_register(a, b);
        Ok(())
    }

    // push: (2 a) -- push <a> onto the stack
    fn push(&mut self) -> Result<(), MachineError> {
        let a = self.read_value()?;
        self.stack.push(a);
        Ok(())
    }

    // pop: (3 a) -- remove the top element from the stack and write it into <a>; empty stack = error
    fn pop(&mut self) -> Result<(), MachineError> {
        let a = self.read_register_idx()?;
        if let Some(value) = self.stack.pop() {
            self.write_register(a, value);
            Ok(())
        } else {
            Err(MachineError::PopOnEmptyStack)
        }
    }

    // eq: (4 a b c) -- set <a> to 1 if <b> is equal to <c>; set it to 0 otherwise
    fn eq(&mut self) -> Result<(), MachineError> {
        let (a, b, c) = self.read_register_idx_binary_args()?;
        self.write_register(a, if b == c { 1 } else { 0 });
        Ok(())
    }

    // gt: (5 a b c) -- set <a> to 1 if <b> is greater than <c>; set it to 0 otherwise
    fn gt(&mut self) -> Result<(), MachineError> {
        let (a, b, c) = self.read_register_idx_binary_args()?;
        self.write_register(a, if b > c { 1 } else { 0 });
        Ok(())
    }

    // jmp: (6 a) -- jump to <a>
    fn jmp(&mut self) -> Result<(), MachineError> {
        let jmp_addr = self.read_next()?;
        self.cp = jmp_addr as usize;
        Ok(())
    }

    // jt: (7 a b) -- if <a> is nonzero, jump to <b>
    fn jt(&mut self) -> Result<(), MachineError> {
        let a = self.read_value()?;
        let b = self.read_next()? as usize;
        if a != 0 {
            self.cp = b;
        };
        Ok(())
    }

    // jf: (8 a b) -- if <a> is zero, jump to <b>
    fn jf(&mut self) -> Result<(), MachineError> {
        let a = self.read_value()?;
        let b = self.read_next()? as usize;
        if a == 0 {
            self.cp = b;
        };
        Ok(())
    }

    // add: (9 a b c) -- assign into <a> the sum of <b> and <c> (modulo 32768)
    fn add(&mut self) -> Result<(), MachineError> {
        let (a, b, c) = self.read_register_idx_binary_args()?;
        self.write_register(a, (b + c) % REGISTERS_OFFSET as u16);
        Ok(())
    }

    // mult: (10 a b c) -- store into <a> the product of <b> and <c> (modulo 32768)
    fn mult(&mut self) -> Result<(), MachineError> {
        let (a, b, c) = self.read_register_idx_binary_args()?;
        self.write_register(a, (b as usize * c as usize % REGISTERS_OFFSET) as u16);
        Ok(())
    }

    // mod: (11 a b c) -- store into <a> the remainder of <b> divided by <c>
    fn mod_op(&mut self) -> Result<(), MachineError> {
        let (a, b, c) = self.read_register_idx_binary_args()?;
        self.write_register(a, b % c);
        Ok(())
    }

    // and: (12 a b c) -- stores into <a> the bitwise and of <b> and <c>
    fn and(&mut self) -> Result<(), MachineError> {
        let (a, b, c) = self.read_register_idx_binary_args()?;
        self.write_register(a, b & c);
        Ok(())
    }

    // or: (13 a b c) -- stores into <a> the bitwise or of <b> and <c>
    fn or(&mut self) -> Result<(), MachineError> {
        let (a, b, c) = self.read_register_idx_binary_args()?;
        self.write_register(a, b | c);
        Ok(())
    }

    // not: (14 a b) -- stores 15-bit bitwise inverse of <b> in <a>
    fn not(&mut self) -> Result<(), MachineError> {
        let (a, b) = self.read_register_idx_unary_arg()?;
        self.write_register(a, !b & 0x7fff);
        Ok(())
    }

    // rmem: (15 a b) -- read memory at address <b> and write it to <a>
    fn rmem(&mut self) -> Result<(), MachineError> {
        let (a, b) = self.read_register_idx_unary_arg()?;
        let value = self.read_memory_at(b as usize)?;
        self.write_register(a, value);
        Ok(())
    }

    // wmem: (16 a b) -- write the value from <b> into memory at address <a>
    fn wmem(&mut self) -> Result<(), MachineError> {
        let a = self.read_value()? as usize; // addr!
        let b = self.read_value()?;
        self.write_memory_at(a, b)
    }

    // call: (17 a) -- write the address of the next instruction to the stack and jump to <a>
    fn call(&mut self) -> Result<(), MachineError> {
        let jmp_addr = self.read_value()?;
        self.stack.push(self.cp as u16);
        self.cp = jmp_addr as usize;
        Ok(())
    }

    // ret: (18) -- remove the top element from the stack and jump to it; empty stack = halt
    fn ret(&mut self) -> Result<(), MachineError> {
        if let Some(jmp_addr) = self.stack.pop() {
            self.cp = jmp_addr as usize;
            Ok(())
        } else {
            Err(MachineError::PopOnEmptyStack)
        }
    }

    // out: (19 a) -- write the character represented by ascii code <a> to the terminal
    fn out(&mut self) -> Result<(), MachineError> {
        let arg = self.read_value()? as u8 as char;
        print!("{arg}");
        Ok(())
    }

    // in: (20 a) -- read a character from the terminal and write its ascii code to <a>
    fn in_op(&mut self) -> Result<(), MachineError> {
        if self.input_buffer.is_empty() {
            let mut buffer = String::new();
            if let Err(error) = io::stdin().read_line(&mut buffer) {
                return Err(MachineError::InputBufferError(error));
            }
            for byte in buffer.as_bytes().iter().rev() {
                self.input_buffer.push(*byte);
            }
        }
        if let Some(ascii) = self.input_buffer.pop() {
            let a = self.read_register_idx()?;
            self.write_register(a, ascii as u16);
            Ok(())
        } else {
            Err(MachineError::EmptyInputBuffer)
        }
    }

    // noop: (21) -- no operation
    fn noop(&self) -> Result<(), MachineError> {
        // no op
        Ok(())
    }
}

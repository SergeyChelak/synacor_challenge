use std::io;
use super::common::*;
use std::collections::HashMap;

pub struct Disassembler {
    memory: [u16; MEMORY_SIZE],
    asm_code: HashMap<usize, String>, // @address, "text representation"
    code_ptr: usize,                  // code pointer
    stack: Vec<usize>,                // callback addresses
}

impl Disassembler {
    pub fn new(program: &Vec<u8>) -> Result<Self, MachineError> {
        Ok(Self {
            memory: setup_memory(program)?,
            asm_code: HashMap::new(),
            code_ptr: 0,
            stack: Vec::new(),
        })
    }

    pub fn parse(&mut self) {
        self.stack.clear();
        self.code_ptr = 0;
        loop {
            let op_code = self.get_next();
            let is_next_instr_allowed = match op_code {
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
                _ => panic!("Debugger: unexpected opcode {op_code}"),
            };
            if is_next_instr_allowed {
                if self.code_ptr >= MEMORY_SIZE {
                    break;
                }
            } else if let Some(ptr) = self.stack.pop() {
                self.code_ptr = ptr;
            } else {
                break;
            }
        }
    }

    #[inline]
    fn get_next(&mut self) -> u16 {
        let v = self.memory[self.code_ptr];
        self.code_ptr += 1;
        v
    }

    fn halt(&mut self) -> bool {
        self.asm_code.insert(self.code_ptr, "halt".to_string());
        self.code_ptr += 1;
        // can't go further
        false
    }

    fn set(&mut self) -> bool {
        panic!("set");
        true
    }

    fn push(&mut self) -> bool {
        panic!("push");
        true
    }

    fn pop(&mut self) -> bool {
        panic!("pop");
        true
    }

    fn eq(&mut self) -> bool {
        panic!("eq");
        true
    }

    fn gt(&mut self) -> bool {
        panic!("gt");
        true
    }

    fn jmp(&mut self) -> bool {
        panic!("jmp");
        true
    }

    fn jt(&mut self) -> bool {
        panic!("jt");
        true
    }

    fn jf(&mut self) -> bool {
        panic!("jf");
        true
    }

    fn add(&mut self) -> bool {
        panic!("add");
        true
    }

    fn mult(&mut self) -> bool {
        panic!("mult");
        true
    }

    fn mod_op(&mut self) -> bool {
        panic!("mod");
        true
    }

    fn and(&mut self) -> bool {
        panic!("and");
        true
    }

    fn or(&mut self) -> bool {
        panic!("or");
        true
    }

    fn not(&mut self) -> bool {
        panic!("not");
        true
    }

    fn rmem(&mut self) -> bool {
        panic!("rmem");
        true
    }

    fn wmem(&mut self) -> bool {
        panic!("wmem");
        true
    }

    fn call(&mut self) -> bool {
        panic!("call");
        true
    }

    fn ret(&mut self) -> bool {
        panic!("ret");
        true
    }

    fn out(&mut self) -> bool {
        panic!("out");
        true
    }

    fn in_op(&mut self) -> bool {
        panic!("in");
        true
    }

    fn noop(&mut self) -> bool {
        self.asm_code.insert(self.code_ptr, "noop".to_string());
        true
    }

    // --
    pub fn save_to_file(&self, file_path: &str) -> io::Result<()> {
        todo!()
    }
}

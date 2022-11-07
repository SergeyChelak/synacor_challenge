use std::io;

pub const REGISTERS_COUNT: usize = 8;
pub const REGISTERS_OFFSET: usize = 32768;
pub const MEMORY_SIZE: usize = 1 << 15; // words

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
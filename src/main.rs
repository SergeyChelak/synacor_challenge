use std::fs::File;
use std::io::{self, Error, ErrorKind, Read, BufRead};

mod machine;
use machine::machine::Machine;

use crate::machine::disasm::{Disassembler, self};
// use machine::debugger::Debugger;

fn main() -> io::Result<()> {
    println!("-- Virtual Machine for Synacor Challenge");
    let args: Vec<String> = std::env::args().collect();
    if let None = args.get(1) {
        show_usage(&args[0]);
        return Err(Error::new(ErrorKind::Other, "Invalid arguments"));
    }
    let program = load_program(&args[1])?;
    println!("{} bytes read", program.len());

    let disasm = Disassembler::new(&program);
    if let Err(error) = disasm {
        println!("Failed to setup disassembler with error {:?}", error);
        return Ok(());
    }
    let mut disasm = disasm.unwrap();
    disasm.parse();
    disasm.save_to_file("data/challenge.asm")?;

    let machine = Machine::new(&program);
    if let Err(error) = machine {
        println!("Failed to setup machine with error {:?}", error);
        return Ok(());
    }
    let mut machine = machine.unwrap();
    // setting up optional script
    if let Some(path) = args.get(2) {
        let mut script: Vec<String> = Vec::new();
        load_script(path, &mut script)?;
        println!("{} commands loaded", script.len());
        //
        machine.write_to_input_buffer(&script);
    }

    println!();
    //machine.run();
    println!("\n-- Execution completed");
    Ok(())
}

fn load_program(path_to_binary: &String) -> io::Result<Vec<u8>> {
    let mut file = File::open(path_to_binary)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    Ok(buffer)
}

fn load_script(path_to_script: &String, script: &mut Vec<String>) -> io::Result<()> {
    let file = File::open(path_to_script)?;
    let lines = io::BufReader::new(file).lines();
    for line in lines {
        if let Ok(str) = line {
            script.push(str);
        }
    }
    Ok(())
}

fn show_usage(path: &String) {
    println!("usage:");
    println!("\t{} <path-to-the challenge.bin>", path);
}
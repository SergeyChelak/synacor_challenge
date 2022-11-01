use std::fs::File;
use std::io::{self, Error, ErrorKind, Read};

mod machine;

use machine::Machine;

fn main() -> io::Result<()> {
    println!("* Rusty Virtual Machine *");
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        show_usage(&args[0]);
        return Err(Error::new(ErrorKind::Other, "Invalid arguments"));
    }
    let program = load_program(&args[1])?;
    println!("{} bytes read", program.len());
    println!();
    let mut machine = Machine::new(program);
    machine.run();
    println!("\n* Goodbye *");
    Ok(())
}

fn load_program(path_to_binary: &String) -> io::Result<Vec<u8>> {
    let mut file = File::open(path_to_binary)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    Ok(buffer)
}

fn show_usage(path: &String) {
    println!("usage:");
    println!("\t{} <path-to-the challenge.bin>", path);
}
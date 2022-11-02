# Virtual Machine
The Rust implementation of the virtual machine for the [Synacor Challenge](https://challenge.synacor.com) <br />

## Brief specs
Details of VM architecture you can find in the challenge <br />
- unlimited stack
- 8 registers
- RAM with address space 0..2<sup>15</sup>
- 21 operations

Executable binary is included

## Usage
Make sure that Cargo and Rust are installed. Then execute in your command line
```
cargo run -- data/challenge.bin
```

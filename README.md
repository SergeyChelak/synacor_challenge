# [Synacor Challenge](https://challenge.synacor.com)
This project is a usual CLI application written in Rust. It also contains additional folders:
- **data** contains the executable binary for Virtual Machine and script with the full list of commands to complete the challenge
- **playground** contains tiny CLI apps that were written during solving quest's tasks

## Virtual Machine

The whole challenge is messing around with a virtual machine. In the very beginning, you should implement 22 instructions according to the architecture specifications. That's not sound like a pretty complex task but it requires a accuracy in this routine.<br/>
Once the virtual machine is completed, you may start it and play the text-based adventure game. Hello from 80th)<br/>

## The Adventure Game
Make sure that Cargo and Rust are installed. Then execute in your command line
```
cargo run -- data/challenge.bin
```

## Coins
The bruteforce solution to order the coins is [here](https://github.com/SergeyChelak/virtual_machine/blob/master/playground/coins/main.rs)

## Teleporter
I needed to write a disassembly and execution dump for the virtual machine to obtain the entry point, conditions to activate the handheld teleporter, and bypass its confirmation mechanism. There is a separate [branch](https://github.com/SergeyChelak/virtual_machine/tree/disassembly) for that purpose. The result is looks something like this:
```
5483          set [r0] 4
5486          set [r1] 1
5489         call @6027
6027           jt [r0] @6035
6030          add [r0] [r1]  1
6034          ret
6035           jt [r1] @6048
6038          add [r0] [r0]  32767
6042          set [r1] [r7]
6045         call @6027
6047          ret
6048         push [r0]
6050          add [r1] [r1]  32767
6054         call @6027           
6056          set [r1] [r0]       
6059          pop [r0]
6061          add [r0] [r0]  32767
6065         call @6027           
6067          ret
```
The search for valid value for the 8th register is [here](https://github.com/SergeyChelak/virtual_machine/blob/master/playground/confirmation/main.rs). The virtual machine patches the program before execution to pass the confirmation mechanism and, of course, to obtain the valid next challenge code

## Vault
You need to change the orb's weight from 22 to 30 by moving among the rooms. The path should be the shortest. The backtracking search for this task is [here](https://github.com/SergeyChelak/virtual_machine/blob/master/playground/vault/main.rs). It isn't a brilliant implementation but good enough to pass this task and complete the challenge.

## Whole challenge
This is a great spoiler on how to pass the challenge and obtain all codes (except the first one):
```
cargo run -- data/challenge.bin data/script.txt
```
Enjoy)

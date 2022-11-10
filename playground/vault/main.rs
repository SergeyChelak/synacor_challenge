use std::collections::HashSet;

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hasher};

#[derive(Clone, Copy)]
enum Op {
    Add,
    Sub,
    Mul,
}

impl Op {
    fn hash(&self) -> u8 {
        match self {
            Self::Add => 1,
            Self::Sub => 2,
            Self::Mul => 3,
        }
    }

    fn to_string(&self) -> String {
        match self {
            Self::Add => "+",
            Self::Sub => "-",
            Self::Mul => "*",
        }.to_string()
    }
}

enum Elem {
    Num(i32),
    Opr(Op),
}

struct Position {
    row: i32,
    col: i32,
    direction: String,
}

impl Position {
    fn new(row: i32, col: i32, direction: &str) -> Self {
        Self {
            row,
            col,
            direction: direction.to_string(),
        }
    }
}

const TARGET: i32 = 30;

fn find(vault_lock: &[[Elem; 4]; 4], row: i32, col: i32, oper: Op, mut result: i32, 
                        viewed: &mut HashSet<u64>, trace: &mut Vec<String>) {    
    let number = as_number(vault_lock, row, col);
    if row == 0 && col == 0 {
        result = number;
        trace.push("----".to_string());
    } else {    
        let key = make_key(&oper, row, col);
        viewed.insert(key);
        result = match oper {
            Op::Add => result + number,
            Op::Sub => result - number,
            Op::Mul => result * number,
        };
    }
    if result < 0 {
        return;
    }
    if row == 3 && col == 3 {
        if result == TARGET {
            println!("Found result:{}\n", trace.join("\n"));
        }
        return;
    }
    let reachable = [   // (number, oper)
        (Position::new(row, col - 2, "west"), Position::new(row, col - 1, "west")),  // 1
        (Position::new(row - 2, col, "south"), Position::new(row - 1, col, "south")),  // 2
        (Position::new(row, col + 2, "east"), Position::new(row, col + 1, "east")),  // 3
        (Position::new(row + 2, col, "north"), Position::new(row + 1, col, "north")),  // 4
        
        (Position::new(row - 1, col - 1, "west"), Position::new(row - 1, col, "south")),  // 5
        (Position::new(row - 1, col - 1, "south"), Position::new(row, col - 1, "west")),  // 6

        (Position::new(row - 1, col + 1, "east"), Position::new(row - 1, col, "south")),  // 7
        (Position::new(row - 1, col + 1, "south"), Position::new(row, col + 1, "east")),  // 8

        (Position::new(row + 1, col + 1, "east"), Position::new(row + 1, col, "north")),  // 9
        (Position::new(row + 1, col + 1, "north"), Position::new(row, col + 1, "east")),  // 10

        (Position::new(row + 1, col - 1, "west"), Position::new(row + 1, col, "north")),  // 11
        (Position::new(row + 1, col - 1, "north"), Position::new(row, col - 1, "west")),  // 12
    ];
    for r in reachable {
        let (dest, next_oper) = r;
        let is_valid = !(dest.row == 0 && dest.col == 0);
        let is_valid = is_valid && dest.row >= 0 && dest.col >= 0 && dest.row < 4 && dest.col < 4;
        if !is_valid { continue; }
        let next_op = as_operation(vault_lock, next_oper.row, next_oper.col);
        let key = make_key(&next_op, dest.row, dest.col);
        let key_revert = key;
        //if !viewed.contains(&key) {
        if trace.len() < 7 {
            let format = format_step(&next_oper, &dest);
            trace.push(format);
            find(vault_lock, dest.row, dest.col, next_op, result, viewed, trace);
            viewed.remove(&key_revert);
            _ = trace.pop();
        }        
    }
}


fn as_number(vault_lock: &[[Elem; 4]; 4], row: i32, col: i32) -> i32 {
    if let Elem::Num(number) = vault_lock[row as usize][col as usize] {
        number
    } else {
        panic!("Not a number")
    }
}

fn as_operation(vault_lock: &[[Elem; 4]; 4], row: i32, col: i32) -> Op {
    if let Elem::Opr(operation) = vault_lock[row as usize][col as usize] {
        operation
    } else {
        panic!("Not an operation")
    }
}

fn make_key(op: &Op, row: i32, col: i32) -> u64 {
    let mut hasher = DefaultHasher::new();
    hasher.write_u8(op.hash());
    hasher.write_i32(row);
    hasher.write_i32(col);
    hasher.finish()
}

fn format_step(op_cell: &Position, number_cell: &Position) -> String {
    format!("{}\n{}", op_cell.direction, number_cell.direction)
}

fn main() {    
    let vault_lock = [
        [Elem::Num(22),      Elem::Opr(Op::Sub), Elem::Num(9),       Elem::Opr(Op::Mul)],
        [Elem::Opr(Op::Add), Elem::Num(4),       Elem::Opr(Op::Sub), Elem::Num(18)],
        [Elem::Num(4),       Elem::Opr(Op::Mul), Elem::Num(11),      Elem::Opr(Op::Mul)],
        [Elem::Opr(Op::Mul), Elem::Num(8),       Elem::Opr(Op::Sub), Elem::Num(1)],
    ];
    let mut viewed: HashSet<u64> = HashSet::new();
    let mut trace: Vec<String> = Vec::new();
    find(&vault_lock, 0, 0, Op::Add, 0, &mut viewed, &mut trace);
    println!("Done");
}

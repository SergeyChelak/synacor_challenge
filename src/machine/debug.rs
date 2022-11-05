use std::fmt::Debug;

#[derive(Debug)]
pub enum DebugToken {
    Address(usize),
    Operation(u16),
    Value(u16, Option<usize>),  // value, optional register index
    RegisterIdx(usize),
    Comment(String),
}

impl DebugToken {
    fn is_comment(&self) -> bool {
        match self {
            Self::Comment(_) => true,
            _ => false,
        }
    }
}

pub trait DebugOutput {
    fn write(&self, tokens: &Vec<DebugToken>);

    fn complete(&self);
}

fn format(tokens: &Vec<DebugToken>) -> String {    
    let fmt_instr = format_instructions(tokens);
    let fmt_comments = format_comments(tokens);
    format!("{:20} {}", fmt_instr, fmt_comments)
}

fn format_instructions(tokens: &Vec<DebugToken>) -> String {
    let instructions: Vec<String> = tokens.iter()
        .filter(|x| !x.is_comment())
        .map(|token|
            match *token {
                DebugToken::Address(address) => format_address(address),
                DebugToken::Operation(op_code) => format_operation(op_code),
                DebugToken::Value(value, reg_idx) => format_value(value, reg_idx),
                DebugToken::RegisterIdx(idx) => format_register_idx(idx),
                _ => "".to_string(),
            }
        )
        .collect();

    instructions.join(" ")
}

fn format_address(address: usize) -> String {
    format!("{:5}", address)
}

fn format_operation(op_code: u16) -> String {
    match op_code {
        0 => "halt",
        1 => "set",
        2 => "push",
        3 => "pop",
        4 => "eq",
        5 => "gt",
        6 => "jmp",
        7 => "jt",
        8 => "jf",
        9 => "add",
        10 => "mult",
        11 => "mod",
        12 => "and",
        13 => "or",
        14 => "not",
        15 => "rmem",
        16 => "wmem",
        17 => "call",
        18 => "ret",
        19 => "out",
        20 => "in",
        21 => "noop",
        _ => "???"
    }.to_string()
}

fn format_value(value: u16, reg_idx_opt: Option<usize>) -> String {
    let mut comps: Vec<String> = Vec::new();
    if let Some(reg_idx) = reg_idx_opt {
        comps.push(format_register_idx(reg_idx));
    }
    comps.push(format!("{value}"));
    comps.join("")
}

fn format_register_idx(idx: usize) -> String {
    format!("[{idx}]")
}

fn format_comments(tokens: &Vec<DebugToken>) -> String {
    let array: Vec<String> = tokens.iter()
        .filter(|x| x.is_comment())
        .map(|x| {
            match x {
                DebugToken::Comment(text) => text.clone(),
                _ => "".to_string(),
            }
        })
        .collect();
    if array.len() > 0 {
        format!("; {}", array.join(", "))
    } else {
        "".to_string()
    }
}
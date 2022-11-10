use std::fmt::Debug;

#[derive(Debug)]
pub enum DebugToken {
    Address(usize),
    Operation(u16),
    Value(u16, Option<usize>), // value, optional register index
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

pub struct TraceFormatter {
    instr_width: usize,
    value_width: usize,
}

impl TraceFormatter {
    pub fn new() -> Self {
        Self {
            instr_width: 50,
            value_width: 9,
        }
    }

    pub fn format(&self, tokens: &Vec<DebugToken>) -> String {
        let fmt_instr = self.format_instructions(tokens);
        let fmt_comments = self.format_comments(tokens);
        format!("{:width$} {}", fmt_instr, fmt_comments, width = self.instr_width)
    }

    fn format_instructions(&self, tokens: &Vec<DebugToken>) -> String {
        let instructions: Vec<String> = tokens
            .iter()
            .filter(|x| !x.is_comment())
            .map(|token| match *token {
                DebugToken::Address(address) => self.format_address(address),
                DebugToken::Operation(op_code) => self.format_operation(op_code),
                DebugToken::Value(value, reg_idx) => self.format_value(value, reg_idx),
                DebugToken::RegisterIdx(idx) => self.format_register_idx(idx),
                _ => "".to_string(),
            })
            .collect();

        instructions.join(" ")
    }

    fn format_address(&self, address: usize) -> String {
        format!("{:<width$}", address, width = self.value_width)
    }

    fn format_operation(&self, op_code: u16) -> String {
        let name = match op_code {
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
            _ => "???",
        };
        format!("{:>7}", name)
    }

    fn format_value(&self, value: u16, reg_idx_opt: Option<usize>) -> String {
        let mut comps: Vec<String> = Vec::new();
        if let Some(reg_idx) = reg_idx_opt {
            comps.push(self.format_register_idx(reg_idx));
        }
        comps.push(format!("{value}"));
        format!("{:<width$}", comps.join(""), width = self.value_width)
    }

    fn format_register_idx(&self, idx: usize) -> String {
        format!("[{idx}]")
    }

    fn format_comments(&self, tokens: &Vec<DebugToken>) -> String {
        let array: Vec<String> = tokens
            .iter()
            .filter(|x| x.is_comment())
            .map(|x| match x {
                DebugToken::Comment(text) => text.clone(),
                _ => "".to_string(),
            })
            .collect();
        if array.len() > 0 {
            format!("; {}", array.join(", "))
        } else {
            "".to_string()
        }
    }
}

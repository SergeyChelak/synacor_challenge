pub enum DebuggerCommand {
    BreakpointsPrint,           // show breakpoint list
    BreakpointAdd(usize),       // add breakpoint at address  
    BreakpointRemove(usize),    // remove breakpoint at address
    BreakpointsEnabled(bool),   // turn on/off breakpoints
    RegistersPrint,             // prints registers state
    RegisterWrite(usize, u16),  // writes u16 value to specified register
    StackSizePrint,             // shows amount of items in stack
    StackPrint,                 // prints stack's values
    TracePrint,                 // prints latest execution trace
    TraceEnabled(bool),         // enables/disables execution trace
    TraceSizePrint,             // prints number of operations in trace
    TraceResize(usize),         // updates trace buffer size (in lines)
    TraceClear,                 // removes all records from trace
    CodePointerPrint,           // prints current code pointer
    CodePointerWrite(usize),    // moves code pointer
    ConsoleClear,               // fills gap with specified amount of lines
    Continue,                   // continue executing
    Unknown,
}

#[derive(Debug)]
enum CommandId {
    BreakpointsPrint,
    BreakpointAdd,
    BreakpointRemove,
    BreakpointsEnabled,
    RegistersPrint,
    RegisterWrite,
    StackSizePrint,
    StackPrint,
    TracePrint,
    TraceEnabled,
    TraceSizePrint,
    TraceResize,
    TraceClear,
    CodePointerPrint,
    CodePointerWrite,
    ConsoleClear,
    Continue
}

enum Rule {
    EqualStr(&'static str),
    AnyNumber,
    AnyBool,
}
enum Parameter {
    Usize(usize),
    Bool(bool),
}

const DBG_CMD_BREAKPOINT: &str = "bp";
const DBG_CMD_REGISTER: &str = "reg";
const DBG_CMD_STACK: &str = "stack";
const DBG_CMD_TRACE: &str = "trace";
const DBG_CMD_CODE_POINTER: &str = "cp";
const DBG_CMD_CONTINUE: &str = "cnt";

const DBG_CMD_ADD: &str = "add";
const DBG_CMD_REMOVE: &str = "rem";
const DBG_CMD_WRITE: &str = "write";
const DBG_CMD_SIZE: &str = "size";
const DBG_CMD_CLEAR: &str = "clear";
const DBG_CMD_TRUE: &str = "true";
const DBG_CMD_FALSE: &str = "false";

pub struct DebugCommandParser {
    rule_registry: Vec<(CommandId, Vec<Rule>)>
}

impl DebugCommandParser {
    pub fn new() -> Self {
        Self {
            rule_registry: Self::create_matching_rules(),
        }
    }

    fn create_matching_rules() -> Vec<(CommandId, Vec<Rule>)> {        
        vec![
            (CommandId::BreakpointsPrint, vec![Rule::EqualStr(DBG_CMD_BREAKPOINT)]),
            (CommandId::BreakpointAdd, vec![Rule::EqualStr(DBG_CMD_BREAKPOINT), Rule::EqualStr(DBG_CMD_ADD), Rule::AnyNumber]),
            (CommandId::BreakpointRemove, vec![Rule::EqualStr(DBG_CMD_BREAKPOINT), Rule::EqualStr(DBG_CMD_REMOVE), Rule::AnyNumber]),
            (CommandId::BreakpointsEnabled, vec![Rule::EqualStr(DBG_CMD_BREAKPOINT), Rule::AnyBool]),                                   

            (CommandId::RegistersPrint, vec![Rule::EqualStr(DBG_CMD_REGISTER)]),
            (CommandId::RegisterWrite, vec![Rule::EqualStr(DBG_CMD_REGISTER), Rule::EqualStr(DBG_CMD_WRITE), Rule::AnyNumber, Rule::AnyNumber]),

            (CommandId::StackSizePrint, vec![Rule::EqualStr(DBG_CMD_STACK), Rule::EqualStr(DBG_CMD_SIZE)]),
            (CommandId::StackPrint, vec![Rule::EqualStr(DBG_CMD_STACK)]),

            (CommandId::TracePrint, vec![Rule::EqualStr(DBG_CMD_TRACE)]),
            (CommandId::TraceEnabled, vec![Rule::EqualStr(DBG_CMD_TRACE), Rule::AnyBool]),
            (CommandId::TraceSizePrint, vec![Rule::EqualStr(DBG_CMD_TRACE), Rule::EqualStr(DBG_CMD_SIZE)]),
            (CommandId::TraceResize, vec![Rule::EqualStr(DBG_CMD_TRACE), Rule::EqualStr(DBG_CMD_SIZE), Rule::AnyNumber]),
            (CommandId::TraceClear, vec![Rule::EqualStr(DBG_CMD_TRACE), Rule::EqualStr(DBG_CMD_CLEAR)]),

            (CommandId::CodePointerPrint, vec![Rule::EqualStr(DBG_CMD_CODE_POINTER)]),                                            
            (CommandId::CodePointerWrite, vec![Rule::EqualStr(DBG_CMD_CODE_POINTER), Rule::EqualStr(DBG_CMD_WRITE), Rule::AnyNumber]),
            
            (CommandId::ConsoleClear, vec![Rule::EqualStr(DBG_CMD_CLEAR)]),
            (CommandId::Continue, vec![Rule::EqualStr(DBG_CMD_CONTINUE)]),
        ]
    }

    pub fn parse(&self, command: &String) -> DebuggerCommand {
        let tokens: Vec<&str> = command.split_whitespace().collect();
        let inp_count = tokens.len();

        'outer: for data in self.rule_registry.iter() {
            let (cmd_id, rules) = data;
            if rules.len() != inp_count {
                continue;
            }            
            let mut params: Vec<Parameter> = Vec::new();
            for pos in 0..inp_count {
                let token = tokens[pos];
                let rule = &rules[pos];
                if !self.is_matching(rule, token, &mut params) {                    
                    continue 'outer;
                }
            }
            return self.build_command(cmd_id, &params);
        }
        DebuggerCommand::Unknown
    }

    fn is_matching(&self, rule: &Rule, token: &str, param_list: &mut Vec<Parameter>) -> bool {
        match *rule {
            Rule::EqualStr(value) => value == token,
            Rule::AnyBool => 
                if token == DBG_CMD_TRUE {
                    param_list.push(Parameter::Bool(true));
                    true
                } else if token == DBG_CMD_FALSE {
                    param_list.push(Parameter::Bool(false));
                    true
                } else {
                    false
                },
            Rule::AnyNumber => {
                if let Ok(value) = token.parse::<usize>() {
                    param_list.push(Parameter::Usize(value));
                    true
                } else {
                    false
                }
            }                
        }
    }

    fn build_command(&self, cmd_id: &CommandId, params: &Vec<Parameter>) -> DebuggerCommand {
        match cmd_id {
            CommandId::BreakpointsPrint => DebuggerCommand::BreakpointsPrint,
            CommandId::BreakpointAdd => 
                if let Parameter::Usize(number) = params[0] {
                    DebuggerCommand::BreakpointAdd(number)
                } else {
                    panic!()
                },
            CommandId::BreakpointRemove => 
                if let Parameter::Usize(number) = params[0] {
                    DebuggerCommand::BreakpointRemove(number)
                } else {
                    panic!()
                },
            CommandId::BreakpointsEnabled =>
                if let Parameter::Bool(is_enabled) = params[0] {
                    DebuggerCommand::BreakpointsEnabled(is_enabled)
                } else {
                    panic!()
                },
            CommandId::RegistersPrint => DebuggerCommand::RegistersPrint,
            CommandId::RegisterWrite =>
                if let (Parameter::Usize(idx), Parameter::Usize(value)) = (&params[0], &params[1]) {
                    DebuggerCommand::RegisterWrite(*idx, *value as u16)
                } else {
                    panic!()
                }
            CommandId::StackSizePrint => DebuggerCommand::StackSizePrint,
            CommandId::StackPrint => DebuggerCommand::StackPrint,
            CommandId::TracePrint => DebuggerCommand::TracePrint,
            CommandId::TraceEnabled => 
                if let Parameter::Bool(is_enabled) = params[0] {
                    DebuggerCommand::TraceEnabled(is_enabled)
                } else {
                    panic!()
                },
            CommandId::TraceSizePrint => DebuggerCommand::TraceSizePrint,
            CommandId::TraceResize =>
                if let Parameter::Usize(number) = params[0] {
                    DebuggerCommand::TraceResize(number)
                } else {
                    panic!()
                },
            CommandId::TraceClear => DebuggerCommand::TraceClear,
            CommandId::CodePointerPrint => DebuggerCommand::CodePointerPrint,
            CommandId::CodePointerWrite =>
                if let Parameter::Usize(number) = params[0] {
                    DebuggerCommand::CodePointerWrite(number)
                } else {
                    panic!()
                },
            CommandId::ConsoleClear => DebuggerCommand::ConsoleClear,
            CommandId::Continue => DebuggerCommand::Continue,
        }        
    }


}

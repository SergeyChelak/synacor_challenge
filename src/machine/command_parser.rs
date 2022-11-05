pub enum DebuggerCommand {
    BreakpointsPrint,           // show breakpoint list
    BreakpointAdd(usize),       // add breakpoint at address  
    BreakpointRemove(usize),    // remove breakpoint at address
    BreakpointsEnabled(bool),   // turn on/off breakpoints
    RegistersPrint,             // prints registers state
    RegisterWrite(usize, u16),  // writes u16 value to specified register
    StackSizePrint,             // shows amount of items in stack
    StrackPrint,                // prints stack's values
    TracePrint,                 // prints latest execution trace
    TraceResize(usize),         // updates trace buffer size (in lines)
    TraceClear,                 // removes all records from trace
    CodePointerPrint,           // prints current code pointer
    CodePointerWrite(usize),    // moves code pointer
    ConsoleClear,               // fills gap with specified amount of lines
    Continue,                   // continue executing
    Unknown,
}

enum CommandId {
    BreakpointsPrint,
    BreakpointAdd,
    BreakpointRemove,
    BreakpointsEnabled,
    RegistersPrint,
    RegisterWrite,
    StackSizePrint,
    StrackPrint,
    TracePrint,
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

const DBG_CMD_LIST: &str = "list";
const DBG_CMD_ADD: &str = "add";
const DBG_CMD_REMOVE: &str = "rem";
const DBG_CMD_WRITE: &str = "write";
const DBG_CMD_SIZE: &str = "size";
const DBG_CMD_CLEAR: &str = "cls";

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
            (CommandId::BreakpointsPrint, vec![Rule::EqualStr(DBG_CMD_BREAKPOINT), Rule::EqualStr(DBG_CMD_LIST)]),
            (CommandId::BreakpointAdd, vec![Rule::EqualStr(DBG_CMD_BREAKPOINT), Rule::EqualStr(DBG_CMD_ADD), Rule::AnyNumber]),
            (CommandId::BreakpointRemove, vec![Rule::EqualStr(DBG_CMD_BREAKPOINT), Rule::EqualStr(DBG_CMD_REMOVE), Rule::AnyNumber]),
            (CommandId::BreakpointsEnabled, vec![Rule::EqualStr(DBG_CMD_BREAKPOINT), Rule::AnyBool]),                                   

            (CommandId::RegistersPrint, vec![Rule::EqualStr(DBG_CMD_REGISTER), Rule::EqualStr(DBG_CMD_LIST)]),
            (CommandId::RegisterWrite, vec![Rule::EqualStr(DBG_CMD_REGISTER), Rule::EqualStr(DBG_CMD_WRITE), Rule::AnyNumber]),

            (CommandId::StackSizePrint, vec![Rule::EqualStr(DBG_CMD_STACK), Rule::EqualStr(DBG_CMD_SIZE)]),
            (CommandId::StrackPrint, vec![Rule::EqualStr(DBG_CMD_STACK)]),

            (CommandId::TracePrint, vec![Rule::EqualStr(DBG_CMD_TRACE)]),
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

        for data in self.rule_registry.iter() {
            let (cmd_id, rules) = data;
            if rules.len() != inp_count {
                continue;
            }            
            for pos in 0..inp_count {
                let token = tokens[pos];
                let rule = &rules[pos];
                if let Some(params) = self.check_matching(rule, token) {
                    return self.build_command(cmd_id, &params);
                }
            }
        }
        DebuggerCommand::Unknown
    }

    fn check_matching(&self, rule: &Rule, token: &str) -> Option<Vec<Parameter>> {
        todo!()
    }

    fn build_command(&self, cmd_id: &CommandId, params: &Vec<Parameter>) -> DebuggerCommand {
        todo!()
    }


}

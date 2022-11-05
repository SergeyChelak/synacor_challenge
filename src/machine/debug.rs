pub enum DebugToken {
    Address(usize),
    Operation(u16),
    Value(u16, Option<usize>),  // value, optional register index
    RegisterIdx(usize),
    Comment(String),
}

pub trait DebugOutput {
    fn write(&self, tokens: &Vec<DebugToken>);

    fn complete(&self);
}
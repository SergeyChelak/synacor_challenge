pub enum DebugToken {
    Address(usize),
    Operation(u16),
    Value(u16, Option<usize>),  // value, optional register index
    RegisterIdx(usize),
    Comment(String),
    EOP                         // end of operation
}
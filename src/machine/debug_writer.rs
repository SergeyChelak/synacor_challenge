use super::debug::*;

// fake writer
pub struct StubDebugWriter {
    formatter: DebugFormatter,
}

impl StubDebugWriter {
    pub fn new() -> Self {
        Self {
            formatter: DebugFormatter::new(),
        }
    }
}

impl DebugOutput for StubDebugWriter {
    fn write(&self, tokens: &Vec<DebugToken>) {
        println!("{}", self.formatter.format_debug_tokens(tokens));
    }

    fn complete(&self) {
        //
    }
}

pub struct FileDebugWriter {

}

impl DebugOutput for FileDebugWriter {
    fn write(&self, tokens: &Vec<DebugToken>) {
        todo!()
    }

    fn complete(&self) {
        todo!()
    }
}
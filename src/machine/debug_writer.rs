use super::debug::*;

// fake writer
pub struct FileDebugWriter {
    formatter: DebugFormatter,
}

impl FileDebugWriter {
    pub fn new() -> Self {
        Self {
            formatter: DebugFormatter::new(),
        }
    }
}

impl DebugOutput for FileDebugWriter {
    fn write(&self, tokens: &Vec<DebugToken>) {
        println!("{}", self.formatter.format_debug_tokens(tokens));
    }

    fn complete(&self) {
        //
    }
}
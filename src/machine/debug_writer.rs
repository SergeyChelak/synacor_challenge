use super::debug::*;

// fake writer
pub struct StubDebugWriter {

}

impl StubDebugWriter {
    pub fn new() -> Self {
        Self {
            //
        }
    }
}

impl DebugOutput for StubDebugWriter {
    fn write(&self, tokens: &Vec<DebugToken>) {
        //
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
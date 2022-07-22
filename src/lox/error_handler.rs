
pub struct ErrorHandler {
    had_error: bool
}

impl ErrorHandler {
    pub fn new() -> Self {
        ErrorHandler { had_error: false }
    }

    pub fn error(&mut self, line: u32, message: &str) {
        self.report(line, "", message)
    }

    fn report(&mut self, line: u32, r#where: &str, message: &str) {
        println!("[line {}] Error{}: {}", line, r#where, message);
        self.had_error = true
    }

    pub fn had_error(&self) -> bool {
        self.had_error
    }

    pub fn clear_error(&mut self) {
        self.had_error = false
    }
}
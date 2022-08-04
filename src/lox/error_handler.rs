
mod error_internals {
    pub trait Report {
        fn report(&mut self, line: u32, r#where: &str, message: &str);
    }
}

pub trait ErrorHandler: error_internals::Report {
    fn new() -> Self;

    fn error(&mut self, line: u32, message: &str) {
        self.report(line, "", message)
    }
}

pub struct CrashingErrorHandler { }

impl ErrorHandler for CrashingErrorHandler {
    fn new() -> Self { CrashingErrorHandler {} }
}

impl error_internals::Report for CrashingErrorHandler {
    fn report(&mut self, line: u32, r#where: &str, message: &str) {
        println!("[line {}] Error{}: {}", line, r#where, message);

        // If there was an error in the file indicate we didn't exit cleanly
        std::process::exit(exitcode::DATAERR);
    }
}

pub struct ReportingErrorHandler { }

impl ErrorHandler for ReportingErrorHandler {
    fn new() -> Self { ReportingErrorHandler {} }
}

impl error_internals::Report for ReportingErrorHandler {
    fn report(&mut self, line: u32, r#where: &str, message: &str) {
        println!("[line {}] Error{}: {}", line, r#where, message);
    }
}
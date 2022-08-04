use super::error_handler::{CrashingErrorHandler, ErrorHandler, ReportingErrorHandler};

// Open the given file and run its contents
pub fn run_file(path: &str) {
    let read_result = std::fs::read_to_string(path);

    match read_result {
        Err(e) => println!("Attempting to read: `{}` caused the following error:\n\t{}", path, e),
        Ok(file_contents) => {
            // If there is an error while running we want to exit
            let mut error_handler = CrashingErrorHandler::new();
            run(&mut error_handler, &file_contents);
        }
    }
}

// Show a prompt to the user and run their input line by line
pub fn run_prompt() {
    use std::io::{stdin,stdout,Write};
    let stdin = stdin();
    let mut stdout = stdout();
    let mut input = String::new();
    // we don't want to exit in the middle of the user's session if there was an error
    let mut error_handler = ReportingErrorHandler::new();

    loop {
        print!("> ");
        let _ = stdout.flush();
        
        match stdin.read_line(&mut input) {
            Ok(n) => {
                if n == 0 {
                    // If they pressed ctrl+d we get no bytes and exit the loop
                    break;
                }

                run(&mut error_handler, &input.trim());

                input.clear()
            },
            Err(e) => println!("Attempting to read line caused the following error:\n\t{}", e),
        }
    }
}

// Run the code
fn run<T: ErrorHandler>(error_handler: &mut T, source: &str) {
    println!("{}", source);
    // let error_in_code = true;
    // if error_in_code {
    //     error_handler.error(0, "message")
    // }
}

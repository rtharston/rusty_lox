use super::error_handler::ErrorHandler;

// Open the given file and run its contents
pub fn run_file(path: &str) {
    let read_result = std::fs::read_to_string(path);

    match read_result {
        Err(e) => println!("Attempting to read: `{}` caused the following error:\n\t{}", path, e),
        Ok(file_contents) => {
            let mut error_handler = ErrorHandler::new();
            run(&mut error_handler, &file_contents);
            if error_handler.had_error() {
                // If there was an error in the file indicate we didn't exit cleanly
                std::process::exit(exitcode::DATAERR);
            }
        }
    }
}

// Show a prompt to the user and run their input line by line
pub fn run_prompt() {
    use std::io::{stdin,stdout,Write};
    let stdin = stdin();
    let mut stdout = stdout();
    let mut input = String::new();
    let mut error_handler = ErrorHandler::new();

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
                // we don't want to exit in the middle of the user's session if there was an error
                error_handler.clear_error();
                input.clear()
            },
            Err(e) => println!("Attempting to read line caused the following error:\n\t{}", e),
        }
    }
}

// Run the code
fn run(error_handler: &mut ErrorHandler, source: &str) {
    println!("{}", source);
    // let error_in_code = true;
    // if error_in_code {
    //     error_handler.error(0, "message")
    // }
}

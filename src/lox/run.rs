// Open the given file and run its contents
pub fn run_file(path: &str) {
    let read_result = std::fs::read_to_string(path);

    match read_result {
        Err(e) => println!("Attempting to read: `{}` caused the following error:\n\t{}", path, e),
        Ok(file_contents) => run(&file_contents)
    }
}

// Show a prompt to the user and run their input line by line
pub fn run_prompt() {
    use std::io::{stdin,stdout,Write};
    let stdin = stdin();
    let mut stdout = stdout();
    let mut input = String::new();

    loop {
        print!("> ");
        let _ = stdout.flush();
        
        match stdin.read_line(&mut input) {
            Ok(n) => {
                if n == 0 {
                    // If they pressed ctrl+d we get no bytes and exit the loop
                    break;
                }

                run(&input.trim());
                input.clear()
            },
            Err(e) => println!("Attempting to read line caused the following error:\n\t{}", e),
        }
    }
}

// Run the code
fn run(source: &str) {
    println!("{}", source)
}

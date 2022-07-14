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
    run("run prompt")
}

// Run the code
fn run(source: &str) {
    println!("{}", source)
}

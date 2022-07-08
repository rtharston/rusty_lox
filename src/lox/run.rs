// Open the given file and run its contents
pub fn run_file(path: &str) {
    let print_str: String = format!("run file: {path}");
    run(&print_str)
}

// Show a prompt to the user and run their input line by line
pub fn run_prompt() {
    run("run prompt")
}

// Run the code
fn run(source: &str) {
    println!("{}", source)
}

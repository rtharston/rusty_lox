extern crate exitcode;

mod lox;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let num_args = args.len();

    if num_args > 2 {
        println!("Usage: rusty_lox [script]");
        std::process::exit(exitcode::USAGE);
    } else if num_args == 2 {
        lox::run::run_file(&args[1]);
    } else {
        lox::run::run_prompt();
    }
}
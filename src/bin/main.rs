use calcinum::parse_expression;
use std::{env, process};

const EXPECTED_ARGS_LEN: usize = 1;

fn main() {
    let args: Vec<_> = env::args().skip(1).collect();
    validate_argc(args.len());

    match args.first().expect("verified argc > 0").as_str() {
        "--version" | "-v" => {
            println!("{}", env!("CARGO_PKG_VERSION"));
            process::exit(0);
        }
        s => match parse_expression(s) {
            Ok(r) => {
                println!("{r}");
                process::exit(0);
            }
            Err(e) => {
                eprintln!("ERROR parsing expression\n\n{s}\n\n{e}");
                process::exit(1);
            }
        },
    }
}

/// Checks that number of received args matches number of expected args.
fn validate_argc(argc: usize) {
    if argc == 0 {
        eprintln!("Missing argument! Please provide an expression as a string, e.g., '2 + 2'");
        process::exit(1);
    }
    if argc != EXPECTED_ARGS_LEN {
        eprintln!("ERROR expected {EXPECTED_ARGS_LEN} argument(s), got {argc}");
        process::exit(1);
    }
}

#[macro_use]
mod macros;
mod context;

use calcinum::parse_expression;
use context::Context;
use std::{
    env,
    io::{Write, stdin, stdout},
    process,
};

const EXPECTED_ARGS_LEN: usize = 1;

fn main() {
    let args: Vec<_> = env::args().skip(1).collect();

    if args.is_empty() {
        repl_mode();
        process::exit(0);
    }

    if args.len() != EXPECTED_ARGS_LEN {
        eprintln!(
            "ERROR expected {EXPECTED_ARGS_LEN} argument(s), got {}",
            args.len()
        );
        process::exit(1);
    }

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

fn repl_mode() {
    let mut ctx = Context::new();

    // ADD YOUR COMMAND AND DESCRIPTION HERE SO IT
    // IS PRINTED TO CONSOLE SO USERS ARE AWARE OF IT!
    let commands = vec![
        ("clear", "   clears the screen"),
        ("reset", "   resets history"),
        ("exit", "    exits the repl"),
        ("history", " prints available history"),
        ("commands", "prints this message"),
    ];

    print_commands(&commands);

    loop {
        let mut input_buf = String::new();

        ctx.print_prefix();

        stdout().flush().expect("failed to flush stdout");

        stdin()
            .read_line(&mut input_buf)
            .expect("failed to read line");

        let input = input_buf.trim();

        // ADD YOUR COMMAND'S "HANDLER" HERE!
        match input {
            "clear" => clear_screen!(),
            "reset" => ctx.reset(),
            "history" => ctx.print_history(),
            "exit" => break,
            "commands" => print_commands(&commands),
            s => ctx.parse(s),
        }
    }
}

/// Each tuple is : ("command_name", "command_description")
fn print_commands(commands: &Vec<(&str, &str)>) {
    println!("\nCommands:\n");
    for cmd in commands {
        print_magenta!("{}", cmd.0);
        println!("      {}", cmd.1);
    }
    println!();
}

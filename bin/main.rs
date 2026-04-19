#[macro_use]
mod macros;
mod context;
mod formatting;

use context::Context;
use rustyline::DefaultEditor;
use std::{
    env,
    io::{Write, stdout},
    process,
};

const EXPECTED_ARGS_LEN: usize = 1;

fn main() {
    let args: Vec<_> = env::args().skip(1).collect();

    if args.is_empty() {
        repl_mode();
        process::exit(0);
    } else if args.len() != EXPECTED_ARGS_LEN {
        eprintln!(
            "ERROR expected {EXPECTED_ARGS_LEN} argument(s), got {}",
            args.len()
        );
        process::exit(1);
    } else {
        match args.first().expect("verified argc > 0").as_str() {
            "--version" | "-v" => {
                println!("{}", env!("CARGO_PKG_VERSION"));
                process::exit(0);
            }
            s => match calcinum::eval(s) {
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
}

fn repl_mode() {
    let mut rl = DefaultEditor::new().expect("rustyline");
    let mut ctx = Context::new();

    //
    // ADD YOUR COMMAND AND DESCRIPTION HERE SO IT
    // IS PRINTED TO CONSOLE SO USERS ARE AWARE OF IT!
    //
    let mut commands = vec![
        ("clear", "clears the screen"),
        ("commands", "prints this message"),
        ("consts", "prints available constants"),
        ("exit", "exits the repl"),
        ("funcs", "prints available functions"),
        ("help", "prints this message"),
        ("history", "prints available history"),
        ("ops", "prints available operators"),
        ("reset", "resets history"),
    ];

    commands.sort_by_key(|e| e.0);
    println_green!("\n{}", env!("CARGO_PKG_NAME").to_uppercase());
    print_commands(&commands);

    loop {
        let line_num = format_cyan!("@{}", ctx.size() + 1);
        let prompt = format!("[{line_num}]> ");
        let line = rl.readline(&prompt);

        match line {
            Ok(input) => {
                //
                // ADD YOUR COMMAND'S "HANDLER" HERE!
                //
                let input = input.as_str();
                match input {
                    "clear" | "cls" => clear_screen(),
                    "reset" => global_reset(&mut ctx, &mut rl),
                    "history" => ctx.print_history(),
                    "help" | "commands" => print_commands(&commands),
                    "funcs" | "functions" => print_available_functions(),
                    "consts" | "constants" => print_available_constants(),
                    "ops" | "operators" => print_available_operators(),
                    "exit" => break,
                    // do nothing if empty string
                    s if s.is_empty() => {}
                    // this needs to be last!
                    s => ctx.parse_and_eval(s),
                };
                rl.add_history_entry(input).expect("input added to history");
            }
            Err(_) => break,
        }
    }
}

fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
    stdout().flush().expect("stdout flush");
}

fn global_reset(
    ctx: &mut Context,
    rl: &mut rustyline::Editor<(), rustyline::history::FileHistory>,
) {
    ctx.reset();
    _ = rl.clear_history();
}

fn print_available_functions() {
    let mut cli_fns = calcinum::cli_functions();
    cli_fns.sort();
    let str = cli_fns.iter().fold(String::new(), |acc, fc| {
        let g = format_green!("{}", fc.to_lowercase());
        let f = format!("{acc}, {g}");
        if acc.is_empty() { g } else { f }
    });
    println!("  {str}");
}

fn print_available_operators() {
    let mut cli_ops = calcinum::cli_operators();
    cli_ops.sort();
    let str = cli_ops.iter().fold(String::new(), |acc, op| {
        let g = format_green!("{}", op.to_lowercase());
        let f = format!("{acc}, {g}");
        if acc.is_empty() { g } else { f }
    });
    println!("  {str}");
}

fn print_available_constants() {
    let mut cli_consts = calcinum::cli_constants();
    cli_consts.sort();
    let str = cli_consts.iter().fold(String::new(), |acc, cn| {
        let g = format_green!("{}", cn.to_lowercase());
        let f = format!("{acc}, {g}");
        if acc.is_empty() { g } else { f }
    });
    println!("  {str}");
}

/// Each tuple is : ("command_name", "command_description")
fn print_commands(commands: &Vec<(&str, &str)>) {
    let min_spaces = 4;
    let longest_name = commands.iter().fold(0, |acc, (name, _)| {
        let len = name.len();
        if acc >= len { acc } else { len }
    });
    let dbl_at = format_cyan!("@@");
    let at_num = format_cyan!("@n");
    let n = format_cyan!("n");

    println!();
    for cmd in commands {
        let spaces = " ".repeat((longest_name - cmd.0.len()) + min_spaces);
        print_magenta!("{}", cmd.0);
        println!("{spaces}{}", cmd.1);
    }
    println!();
    println!("{dbl_at} : references the result from previous expression");
    println!("{at_num} : references the result from line '{n}'");
    println!();
}

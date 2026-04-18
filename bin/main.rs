#[macro_use]
mod macros;
mod context;

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
    let commands = vec![
        ("clear", "     clears the screen"),
        ("commands", "  prints this message"),
        ("constants", " prints available constants"),
        ("exit", "      exits the repl"),
        ("functions", " prints available functions"),
        ("help", "      prints this message"),
        ("history", "   prints available history"),
        ("operators", " prints available operators"),
        ("reset", "     resets history"),
    ];

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
                    "clear" => clear_screen(),
                    "reset" => global_reset(&mut ctx, &mut rl),
                    "history" => ctx.print_history(),
                    "help" | "commands" => print_commands(&commands),
                    "functions" => print_available_functions(),
                    "constants" => print_available_constants(),
                    "operators" => print_available_operators(),
                    "exit" => break,
                    // this needs to be last!
                    s => ctx.parse_and_eval(s),
                };
                rl.add_history_entry(input).expect("input added to history");
            }
            Err(_) => break,
        }
    }
}

/// Each tuple is : ("command_name", "command_description")
fn print_commands(commands: &Vec<(&str, &str)>) {
    println!("\nCommands:\n");

    let dblat = format_cyan!("@@");
    let atnum = format_cyan!("@n");
    let n = format_cyan!("n");
    println!("'{dblat}' references the result from previous expression");
    println!("'{atnum}' references the result from line '{n}'");
    println!();

    for cmd in commands {
        print_magenta!("{}", cmd.0);
        println!("      {}", cmd.1);
    }
    println!();
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

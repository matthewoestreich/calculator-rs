use std::{iter, str::Chars};

use calcinum::parse_expression;

#[derive(Default, Debug)]
pub struct Context {
    /// (String, Option<String>) = (expression, Some(expression_result) | None if expression produced an error)
    history: Vec<(String, Option<String>)>,
}

impl Context {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn print_prefix(&self) {
        print!("[");
        print_cyan!("@{}", self.history.len() + 1);
        print!("]> ");
    }

    pub fn print_history(&self) {
        for (i, (_, r)) in self.history.iter().enumerate() {
            let i = i + 1;
            let res = r.as_deref().unwrap_or("ERROR");
            println_green!("@{i}\t= {res}");
        }
        println!();
    }

    pub fn reset(&mut self) {
        self.history.clear();
        println_green!("\n--- HISTORY RESET ---\n");
    }

    pub fn parse(&mut self, expression: &str) {
        let mut output = String::new();
        let mut iter = expression.chars().peekable();

        while let Some(c) = iter.next() {
            match c {
                '@' => {
                    let Some(i) = self.parse_history_ref(&mut iter) else {
                        println_red!(
                            "\nUnable to parse provided line. Expected format is '@1' where '1' is the target line.\n"
                        );
                        self.push_history(expression, None);
                        return;
                    };

                    if i == 0 || i > self.history.len() {
                        println_red!("\nLine '{i}' does not exist.\n");
                        self.push_history(expression, None);
                        return;
                    }

                    let Some(val) = self.resolve_history(i) else {
                        println_red!(
                            "\nLine '{i}' had an error result. Error results cannot be used in expressions.\n"
                        );
                        self.push_history(expression, None);
                        return;
                    };

                    output.push_str(val);
                }

                _ => output.push(c),
            }
        }

        self.eval(&output);
    }

    fn eval(&mut self, expression: &str) {
        match parse_expression(expression) {
            Ok(r) => {
                println_green!("\n{r}\n");
                self.push_history(expression, Some(r.to_string()));
            }
            Err(e) => {
                println_red!("\n{expression}\n{e}\n");
                self.push_history(expression, None);
            }
        }
    }

    fn parse_history_ref(&self, iter: &mut iter::Peekable<Chars>) -> Option<usize> {
        let mut num_str = String::new();

        while let Some(&c) = iter.peek() {
            if !c.is_ascii_digit() {
                break;
            }
            num_str.push(c);
            iter.next();
        }

        num_str.parse::<usize>().ok()
    }

    fn resolve_history(&self, i: usize) -> Option<&str> {
        if i == 0 {
            return None;
        }
        let (_, result) = self.history.get(i - 1)?;
        result.as_deref()
    }

    fn push_history(&mut self, expression: &str, result: Option<String>) {
        self.history.push((expression.to_string(), result));
    }
}

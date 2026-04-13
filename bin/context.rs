use std::{iter, str::Chars};

use calcinum::Formatting;

#[derive(Default, Debug)]
pub struct Context {
    /// (String, Option<String>) = (expression, Some(expression_result) | None if expression produced an error)
    history: Vec<(String, Option<String>)>,
}

impl Context {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn size(&self) -> usize {
        self.history.len()
    }

    pub fn print_history(&self) {
        for (i, (e, r)) in self.history.iter().enumerate() {
            let i = i + 1;
            let res = r.as_deref().unwrap_or("ERROR");
            print_green!("@{i}");
            println!("\n  expression = '{e}'\n  result     = '{res}'");
        }
    }

    pub fn reset(&mut self) {
        self.history.clear();
        println_green!("--- HISTORY RESET ---");
    }

    pub fn parse_and_eval(&mut self, expression: &str) {
        let mut output = String::new();
        let mut iter = expression.chars().peekable();
        let mut bin_fmt_sep = String::new();
        let mut bin_fmt_grp = 0;

        while let Some(c) = iter.next() {
            match c {
                ':' => {
                    if let Some(cc) = iter.peek()
                        && *cc == 'b'
                    {
                        iter.next();
                        while let Some(cc) = iter.next() {
                            if iter.peek().is_none() {
                                // curr on last item
                                if cc.is_ascii_digit() {
                                    bin_fmt_grp = cc.to_string().parse::<usize>().unwrap();
                                } else {
                                    bin_fmt_sep.push(cc);
                                }
                            } else {
                                bin_fmt_sep.push(cc);
                            }
                        }
                    }
                }
                '@' => {
                    let Some(i) = self.parse_history_ref(&mut iter) else {
                        println_red!(
                            "Unable to parse provided line. Expected format is '@1' where '1' is the target line."
                        );
                        self.push_history(expression, None);
                        return;
                    };
                    if i == 0 || i > self.history.len() {
                        println_red!("Line '{i}' does not exist.");
                        self.push_history(expression, None);
                        return;
                    }
                    let Some(val) = self.resolve_history(i) else {
                        println_red!(
                            "Line '{i}' had an error result. Error results cannot be used in expressions."
                        );
                        self.push_history(expression, None);
                        return;
                    };
                    output.push_str(val);
                }
                _ => output.push(c),
            }
        }

        let bin_fmt = if bin_fmt_sep.is_empty() {
            None
        } else {
            Some(bin_fmt_sep.as_str())
        };
        self.eval(&output, bin_fmt, bin_fmt_grp);
    }

    fn eval(&mut self, expression: &str, bin_fmt_separator: Option<&str>, bin_fmt_grouping: usize) {
        match calcinum::eval(expression) {
            Ok(r) => {
                println_green!("{r}");
                if let Some(sep) = bin_fmt_separator {
                    let n = r.format(Formatting::Binary {
                        separator: sep.to_string(),
                        grouping: bin_fmt_grouping,
                    });
                    println_green!("{n}");
                }
                self.push_history(expression, Some(r.to_string()));
            }
            Err(e) => {
                let nl = if expression.is_empty() { "" } else { "\n" };
                println_red!("{expression}{nl}{e}");
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

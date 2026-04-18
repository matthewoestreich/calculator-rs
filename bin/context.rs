use crate::formatting::{FormatSpec, Formatter};
use calcinum::{CalculatorError, Number};
use std::{iter, str::Chars};

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
        let mut specifier = String::new();

        while let Some(c) = iter.next() {
            match c {
                ':' => {
                    // Once we see the formatting delimeter, it is safe
                    // to assume we can read the rest of the line as formatting syntax.
                    for fs in iter.by_ref() {
                        specifier.push(fs);
                    }
                }
                '@' => {
                    let i = match self.parse_history_ref(&mut iter) {
                        Ok(n) => n,
                        Err(e) => {
                            println_red!("{}", e.message);
                            self.push_history(expression, None);
                            return;
                        }
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

        self.eval(&output, &specifier);
    }

    /// `expression` is the 'infix' expression, `specifier` is the format specifier string.
    fn eval(&mut self, expression: &str, specifier: &str) {
        match calcinum::eval(expression) {
            Ok(r) => {
                self.push_history(expression, Some(r.to_string()));
                if specifier.is_empty() {
                    // No  formatting was used.
                    println_green!("{r}");
                } else {
                    // Formatting was used.
                    self.format_number_and_print(&r, specifier);
                }
            }
            Err(e) => {
                let nl = if expression.is_empty() { "" } else { "\n" };
                println_red!("{expression}{nl}{e}");
                self.push_history(expression, None);
            }
        }
    }

    fn parse_history_ref(
        &self,
        iter: &mut iter::Peekable<Chars>,
    ) -> Result<usize, CalculatorError> {
        // This means we got `@@`, which means the last history element.
        if let Some(l) = iter.peek()
            && *l == '@'
        {
            if self.history.is_empty() {
                return Err(CalculatorError {
                    message: "Previous history result does not exist.".to_string(),
                });
            }
            iter.next();
            return Ok(self.history.len());
        }

        let mut num_str = String::new();

        while let Some(&c) = iter.peek() {
            if !c.is_ascii_digit() {
                break;
            }
            num_str.push(c);
            iter.next();
        }

        num_str.parse::<usize>().map_err(|_| CalculatorError {
            message: "Unable to parse provided line. Expected format is '@1' where '1' is the target line.".to_string(),
        })
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

    fn format_number_and_print(&self, number: &Number, spec: &str) {
        match FormatSpec::parse(spec) {
            Ok(parsed_spec) => match Formatter::format_number(number, parsed_spec) {
                Ok(formatted) => println_green!("{formatted}"),
                Err(e) => println_yellow!("Invalid format specifier '{spec}' : {e:?}"),
            },
            Err(e) => println_yellow!("Invalid format specifier '{spec}' : {e:?}"),
        }
    }
}

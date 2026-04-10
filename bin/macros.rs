macro_rules! clear_screen {
    () => {{ print!("\x1B[2J\x1B[1;1H") }};
}

macro_rules! println_red {
    ($($arg:tt)*) => {
        println!("\x1b[91m{}\x1b[0m", format_args!($($arg)*));
    };
}

macro_rules! println_green {
    ($($arg:tt)*) => {
        println!("\x1b[92m{}\x1b[0m", format_args!($($arg)*));
    };
}

macro_rules! print_green {
    ($($arg:tt)*) => {
        print!("\x1b[92m{}\x1b[0m", format_args!($($arg)*));
    };
}

macro_rules! print_magenta {
    ($($arg:tt)*) => {
        print!("\x1b[95m{}\x1b[0m", format_args!($($arg)*));
    };
}

macro_rules! format_cyan {
    ($($arg:tt)*) => {
        format!("\x1b[96m{}\x1b[0m", format_args!($($arg)*))
    };
}

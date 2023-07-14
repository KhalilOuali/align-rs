use std::{
    env::{args, Args},
    io::stdin,
    process::exit,
};

use terminal_size::{terminal_size, Width};

enum Where {
    Left,
    Center,
    Right,
}

fn get_width() -> usize {
    if let Some((Width(w), _)) = terminal_size() {
        w.into()
    } else {
        eprintln!("Unable to get terminal width.");
        exit(3);
    }
}

fn get_arg(args: &mut Args) -> Where {
    if let Some(arg) = args.next() {
        match arg.as_str() {
            "left" | "l" => Where::Left,
            "center" | "c" => Where::Center,
            "right" | "r" => Where::Right,
            _ => {
                eprintln!("Invalid arugment: {arg}.");
                exit(2);
            }
        }
    } else {
        Where::Center
    }
}

/// Reads command line arguments and return the align and justify parameters
fn get_params() -> (Where, Where) {
    let mut args = args();
    if args.len() > 3 {
        eprintln!("Invalid number of arguments.");
        exit(1);
    }
    args.next();

    let align = get_arg(&mut args);
    let justify = get_arg(&mut args);
    (align, justify)
}

/// Returns (space, lines)
/// where lines is a vector contraining the lines of text
/// and space is the amount of space left on the terminal line
fn get_text(width: usize, justify: Where) -> (Vec<String>, usize) {
    let mut lines = Vec::new();
    let mut max_len: usize = 0;

    // unwrap lines and caluclate max line length
    for (i, line) in stdin().lines().enumerate() {
        match line {
            Ok(line) => {
                let len = line.len();
                if len > width {
                    eprintln!("Line {i} is longer than terminal width.");
                    exit(5);
                }
                if len > max_len {
                    max_len = len;
                }
                lines.push(line);
            }
            Err(error) => {
                eprintln!("Unable to read line {i}: {error}.");
                exit(4);
            }
        }
    }

    // add spaces to each line to justify them and make them all same length
    lines.iter_mut().for_each(|line| {
        let space = max_len - line.len();

        let before = match justify {
            Where::Left => 0,
            Where::Center => space / 2,
            Where::Right => space,
        };
        let after = space - before;

        (*line).insert_str(0, " ".repeat(before).as_str());
        (*line).push_str(" ".repeat(after).as_str());
    });

    (lines, width - max_len)
}

fn print_text(lines: Vec<String>, space: usize, align: Where) {
    let space = match align {
        Where::Left => 0,
        Where::Center => space / 2,
        Where::Right => space,
    };

    lines
        .iter()
        .for_each(|line| println!("{}{}", " ".repeat(space), line));
}

fn main() {
    let width = get_width();
    // todo: use an argument pasring library
    // todo: add a trim option
    // todo: add a center bias option
    let (align, justify) = get_params();
    // todo: break apart read and justify
    let (lines, space) = get_text(width, justify);
    // todo: break apart print and aling
    print_text(lines, space, align);
}

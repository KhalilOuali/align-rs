use std::{
    env::{args, Args},
    io::stdin,
    process::exit,
};

use align::{Align, Bias, Where};

fn get_width() -> usize {
    if let Some((w, _)) = term_size::dimensions() {
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
        Where::Left
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

fn get_text() -> Vec<String> {
    stdin()
        .lines()
        .map(|line| line.expect("stdin error: "))
        .collect()
}

fn main() {
    let width = get_width();
    // todo: use an argument pasring library
    // todo: add a trim option
    // todo: add a center bias option
    let (align, justify) = get_params();
    let mut lines = get_text();

    // justify
    if let Err(e) = lines.align_text(justify, None, false, Bias::Left, true) {
        eprintln!("Error: {e:?}");
        exit(1);
    }

    // align
    if let Err(e) = lines.align_text(align, Some(width), false, Bias::Left, true) {
        eprintln!("Error: {e:?}");
        exit(1);
    }

    lines.iter().for_each(|line| println!("{line}"));
}

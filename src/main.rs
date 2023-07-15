use std::{io::stdin, process::exit};

use align::{Align, Bias, Where};

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, long_about = None)]
#[command(about = "Aligns and justifies text within the terminal (or a specified width).")]
struct Args {
    /// Where to align the text.
    #[arg(
        value_enum,
        short,
        long,
        default_value_t,
        ignore_case = true,
        conflicts_with = "align_justify"
    )]
    align: Where,

    /// Where to justify the text.
    #[arg(
        value_enum,
        short,
        long,
        default_value_t,
        ignore_case = true,
        conflicts_with = "align_justify"
    )]
    justify: Where,

    /// Shorthand for specifiying both.
    #[arg(
        value_enum,
        long,
        long = "aj",
        ignore_case = true,
        conflicts_with = "align",
        conflicts_with = "justify"
    )]
    align_justify: Option<Where>,

    /// Whether to trim the spaces around the lines before justifying.
    #[arg(short, long, action)]
    trim: bool,

    // todo: width=0 takes text width
    /// Width to align the text within. If unspecified, takes terminal width.
    #[arg(short, long)]
    width: Option<usize>,

    /// Whether to keep the spaces on the right.
    #[arg(short, long, action)]
    keep_spaces: bool,

    /// Which side to bias towards if line can't be perfectly centered.
    #[arg(value_enum, short, long, default_value_t, ignore_case = true)]
    bias: Bias,
    // todo: add wrapping option
}

fn get_terimnal_width() -> usize {
    if let Some((w, _)) = term_size::dimensions() {
        w.into()
    } else {
        eprintln!("Unable to get terminal width.");
        exit(3);
    }
}

fn get_text() -> Vec<String> {
    stdin()
        .lines()
        .map(|line| line.expect("read error: "))
        .collect()
}

fn main() {
    let mut args = Args::parse();
    if let Some(wh) = args.align_justify {
        args.align = wh.clone();
        args.justify = wh.clone();
    }
    if args.align != Where::Left && args.width.is_none() {
        // todo: come back after width=0 option added
        _ = args.width.insert(get_terimnal_width());
    }

    let mut lines = get_text();

    if args.align == Where::Center && args.justify == Where::Center {
        // center completely
        if let Err(e) = lines.align_text(
            Where::Center,
            Some(args.width.unwrap()),
            args.trim,
            args.bias,
            args.keep_spaces,
        ) {
            eprintln!("Error: {e:?}");
            exit(1);
        }
    } else {
        // justify
        if let Err(e) = lines.align_text(args.justify, None, args.trim, args.bias, true) {
            eprintln!("Error: {e:?}");
            exit(1);
        }
        // todo: if !keep_spaces, remove spaces introduced by justify

        // align
        if let Err(e) = lines.align_text(args.align, args.width, false, args.bias, args.keep_spaces)
        {
            eprintln!("Error: {e:?}");
            exit(1);
        }
    }

    lines.iter().for_each(|line| println!("{line}"));
}

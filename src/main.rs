use std::io::stdin;

use align::*;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, long_about = "None")]
#[command(
    about = "Aligns a block of text within the terminal (or a specified number of columns)."
)]
struct Args {
    /// Where to align the block of text.
    #[arg(
        value_enum,
        short,
        long,
        default_value_t,
        ignore_case = true,
        conflicts_with = "align"
    )]
    outer: Where,

    /// Where to align text inside the block.
    #[arg(
        value_enum,
        short,
        long,
        default_value_t,
        ignore_case = true,
        conflicts_with = "align"
    )]
    inner: Where,

    /// Shorthand for specifiying both.
    #[arg(
        value_enum,
        short,
        long,
        ignore_case = true,
        conflicts_with = "outer",
        conflicts_with = "inner"
    )]
    align: Option<Where>,

    /// Number of columns. Takes text's width if 0, terminal's width if unspecified.
    #[arg(short, long)]
    columns: Option<usize>,

    /// Wrap the lines of text to fit in the number of columns.
    #[arg(short, long, action)]
    wrap: bool,

    /// Trim the spaces around the lines before aligning.
    #[arg(short, long, action)]
    trim: bool,

    /// Keep the spaces on the right in output.
    #[arg(short, long, action)]
    keep: bool,

    /// Offset if line can't be centered perfectly
    #[arg(value_enum, short, long, default_value_t, ignore_case = true)]
    bias: Bias,
}

fn get_terimnal_width() -> Option<usize> {
    term_size::dimensions().map(|(width, _height)| width)
}

fn get_text() -> Vec<String> {
    stdin().lines().map(|line| line.unwrap()).collect()
}

fn main() -> Result<(), &'static str> {
    let mut args = Args::parse();
    if let Some(wh) = args.align {
        args.outer = wh.clone();
        args.inner = wh.clone();
    }

    // deduce final number of columns depending on args
    let cols_wrap = match args.columns {
        None => match get_terimnal_width() {
            Some(term_width) => Some((term_width, args.wrap)),
            None => {
                return Err("couldn't get terminal width");
            }
        },
        Some(0) => None,
        Some(c) => Some((c, args.wrap)),
    };

    let mut lines = get_text();

    if args.outer == Where::Center && args.inner == Where::Center {
        // center completely
        if let Err(Error::InsufficientColumns) =
            lines.align_text(Where::Center, cols_wrap, args.trim, args.bias, args.keep)
        {
            return Err("not enough columns");
        }
    } else {
        // inner align
        lines
            .align_text(args.inner, None, args.trim, args.bias, true)
            .unwrap();

        // outer align
        if let Err(Error::InsufficientColumns) =
            lines.align_text(args.outer, cols_wrap, false, args.bias, args.keep)
        {
            return Err("not enough columns");
        }

        if !args.keep {
            // remove spaces introduced in inner align
            lines
                .iter_mut()
                .for_each(|line| *line = line.trim_end().to_string());
        }
    }

    lines.iter().for_each(|line| println!("{line}"));

    Ok(())
}

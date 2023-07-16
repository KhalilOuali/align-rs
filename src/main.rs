use std::io::stdin;

use align::*;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, long_about = "None")]
#[command(
    about = "Aligns and justifies text within the terminal (or a specified number of columns)."
)]
struct Args {
    /// Where to align the text.
    #[arg(
        value_enum,
        short,
        long,
        default_value_t,
        ignore_case = true,
        conflicts_with = "both"
    )]
    align: Where,

    /// Where to justify the text.
    #[arg(
        value_enum,
        short,
        long,
        default_value_t,
        ignore_case = true,
        conflicts_with = "both"
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
    both: Option<Where>,

    /// Number of columns. Takes text's width if 0, terminal's width if unspecified.
    #[arg(short, long)]
    columns: Option<usize>,

    /// Wrap the lines of text to fit in the number of columns.
    #[arg(short, long, action)]
    wrap: bool,

    /// Trim the spaces around the lines before justifying.
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
    if let Some(wh) = args.both {
        args.align = wh.clone();
        args.justify = wh.clone();
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

    if args.align == Where::Center && args.justify == Where::Center {
        // center completely
        if let Err(Error::InsufficientColumns) =
            lines.align_text(Where::Center, cols_wrap, args.trim, args.bias, args.keep)
        {
            return Err("not enough columns");
        }
    } else {
        // justify
        lines
            .align_text(args.justify, None, args.trim, args.bias, true)
            .unwrap();

        // align
        if let Err(Error::InsufficientColumns) =
            lines.align_text(args.align, cols_wrap, false, args.bias, args.keep)
        {
            return Err("not enough columns");
        }

        if !args.keep {
            // remove spaces introduced by justify
            lines
                .iter_mut()
                .for_each(|line| *line = line.trim_end().to_string());
        }
    }

    lines.iter().for_each(|line| println!("{line}"));

    Ok(())
}

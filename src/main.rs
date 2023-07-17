use std::io::stdin;

use align::*;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, long_about = "None")]
#[command(about = "Aligns a block of text within the terminal (or a specified number of columns).")]
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

fn get_terimnal_width() -> Result<usize, String> {
    term_size::dimensions()
        .map(|(width, _height)| width)
        .ok_or("couldn't get terminal width".to_string())
}

fn get_text() -> Result<Vec<String>, String> {
    stdin()
        .lines()
        .map(|line| line.map_err(|e| e.to_string()))
        .collect()
}

fn main() -> Result<(), String> {
    let mut args = Args::parse();
    if let Some(wh) = args.align {
        args.outer = wh.clone();
        args.inner = wh;
    }

    // deduce final number of columns depending on args
    let cols_wrap = match args.columns {
        None => Some((get_terimnal_width()?, args.wrap)),
        Some(0) => None,
        Some(c) => Some((c, args.wrap)),
    };

    let mut lines = get_text()?;

    if args.outer == Where::Center && args.inner == Where::Center {
        // center completely
        lines = lines
            .align_text(Where::Center, cols_wrap, args.trim, args.bias, args.keep)
            .map_err(|e| e.to_string())?;
    } else {
        // inner align
        lines = lines
            .align_text(args.inner, None, args.trim, args.bias, true)
            .map_err(|e| e.to_string())?;

        // outer align
        lines = lines
            .align_text(args.outer, cols_wrap, false, args.bias, args.keep)
            .map_err(|e| e.to_string())?;

        if !args.keep {
            // remove spaces introduced in inner align
            lines
                .iter_mut()
                .for_each(|line| *line = line.trim_end().to_string());
        }
    }

    for line in lines {
        println!("{line}");
    }

    Ok(())
}

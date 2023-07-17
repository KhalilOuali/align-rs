use std::fmt::Display;

use clap::ValueEnum;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Where {
    Left,
    Center,
    Right,
}

impl Default for Where {
    fn default() -> Self {
        Where::Left
    }
}

impl ValueEnum for Where {
    fn from_str(input: &str, ignore_case: bool) -> Result<Self, String> {
        let input = if ignore_case {
            input.to_lowercase()
        } else {
            input.to_string()
        };

        match input.as_str() {
            "l" | "left" => Ok(Where::Left),
            "c" | "center" => Ok(Where::Center),
            "r" | "right" => Ok(Where::Right),
            _ => Err("invalid Where value".to_string()),
        }
    }

    fn value_variants<'a>() -> &'a [Self] {
        &[Where::Left, Where::Center, Where::Right]
    }

    fn to_possible_value(&self) -> Option<clap::builder::PossibleValue> {
        match self {
            Where::Left => Some(clap::builder::PossibleValue::new("left").alias("l")),
            Where::Center => Some(clap::builder::PossibleValue::new("center").alias("c")),
            Where::Right => Some(clap::builder::PossibleValue::new("right").alias("r")),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Bias {
    Left,
    Right,
}

impl Default for Bias {
    fn default() -> Self {
        Bias::Left
    }
}

impl ValueEnum for Bias {
    fn from_str(input: &str, ignore_case: bool) -> Result<Self, String> {
        let input = if ignore_case {
            input.to_lowercase()
        } else {
            input.to_string()
        };

        match input.as_str() {
            "l" | "left" => Ok(Bias::Left),
            "r" | "right" => Ok(Bias::Right),
            _ => Err("invalid Bias value".to_string()),
        }
    }

    fn value_variants<'a>() -> &'a [Self] {
        &[Bias::Left, Bias::Right]
    }

    fn to_possible_value(&self) -> Option<clap::builder::PossibleValue> {
        match self {
            Bias::Left => Some(clap::builder::PossibleValue::new("left").alias("l")),
            Bias::Right => Some(clap::builder::PossibleValue::new("right").alias("r")),
        }
    }
}

impl From<Bias> for usize {
    fn from(value: Bias) -> Self {
        match value {
            Bias::Left => 0,
            Bias::Right => 1,
        }
    }
}

/// Errors returned by [`align_text()`]:
/// * [`Error::InsufficientColumns`]: the `lines` can't fit in the given number of `columns`.
/// * [`Error::UnknownError`]: an unexpected error that shouldn't have occured.
///
/// # Example
/// * Passing an insufficient number of columns:
/// ```
/// use align_text::{Align, Where, Bias, Error};
/// 
/// let mut lines = vec!["0123456789".to_string()];
/// let result = lines.align_text(Where::Center, Some((3, false)), true, Bias::Right, true);
/// 
/// assert_eq!(result, Err(Error::InsufficientColumns));
/// ```
#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    InsufficientColumns,
    UnknownError(&'static str),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::InsufficientColumns => write!(f, "text can't fit, not enough columns"),
            Error::UnknownError(e) => write!(f, "unexpected, {e}"),
        }
    }
}

/// The trait which defines the align_text() function.
/// No defaut implementation.
/// Implemented for [`Vec<String>`].
pub trait Align {
    fn align_text(
        &self,
        align: Where,
        columns: Option<(usize, bool)>,
        trim: bool,
        bias: Bias,
        keep_spaces: bool,
    ) -> Result<Self, Error>
    where
        Self: Sized;
}

impl Align for Vec<String> {
    /// Aligns each line of text within a number of columns by inserting spaces to its left and right.
    /// See [`Error`] for potential errors returned.
    /// # Params
    /// * `align`: Where to align the lines.
    /// * `columns`: can be
    ///   * `Some(num, wrap)`: Number of columns and whether to wrap lines which are too long.
    ///   * `None`: Use text's width as number of columns (maximum line length).
    /// * `trim`: Whether to trim white-spaces around the lines before aligment.
    /// * `bias`: Which side to bias towards if line can't be perfectly centered.
    /// * `keep_spaces`: Whether to keep the spaces on the right.
    ///
    /// # Note
    /// This method is designed for use with a vector of single-line strings.
    /// The result may look weird if you have newlines in you text.
    ///
    /// # Examples
    /// ```
    /// use align_text::{Align, Bias, Where};
    /// let text = vec![
    ///     "Hello           ".to_string(),
    ///     "            World!".to_string(),
    ///     "   This should center-align     ".to_string(),
    /// ];
    /// let aligned = text
    ///     .align_text(Where::Center, Some((30, false)), true, Bias::Right, true)
    ///     .unwrap();
    /// assert_eq!(aligned[0], "             Hello            ");
    /// assert_eq!(aligned[1], "            World!            ");
    /// assert_eq!(aligned[2], "   This should center-align   ");
    /// ```
    fn align_text(
        &self,
        align: Where,
        columns: Option<(usize, bool)>,
        trim: bool,
        bias: Bias,
        keep_spaces: bool,
    ) -> Result<Vec<String>, Error> {
        let mut lines = self.clone();

        if lines.is_empty() {
            return Ok(lines);
        }

        if trim {
            lines
                .iter_mut()
                .for_each(|line| *line = line.trim().to_string());
        }

        let text_width = lines
            .iter()
            .map(|line| line.len())
            .max()
            .ok_or(Error::UnknownError("couldn't caluclate text_width"))?;

        let num_cols = match columns {
            None => text_width,
            Some((num, wrap)) if num < text_width => {
                if !wrap {
                    return Err(Error::InsufficientColumns);
                }

                // if wrap, split strings into substrings of length num
                lines = lines
                    .iter()
                    .flat_map(|line| {
                        line.chars()
                            .collect::<Vec<char>>()
                            .chunks(num)
                            .map(|line_chars| line_chars.iter().collect::<String>())
                            .collect::<Vec<String>>()
                    })
                    .collect();

                num
            }
            Some((num, _)) => num,
        };

        // align by adding spaces before and after
        for line in lines.iter_mut() {
            let space = num_cols - line.len();

            let before = match align {
                Where::Left => 0,
                Where::Center => (space + usize::from(bias)) / 2,
                Where::Right => space,
            };
            let after = space - before;

            line.insert_str(0, " ".repeat(before).as_str());

            if keep_spaces {
                line.push_str(" ".repeat(after).as_str());
            }
        }

        Ok(lines)
    }
}

impl Align for String {
    /// Aligns each line of text within a number of columns by inserting spaces to its left and right.
    /// See [`Error`] for potential errors returned.
    /// # Params
    /// * `align`: Where to align the lines.
    /// * `columns`: can be
    ///   * `Some(num, wrap)`: Number of columns and whether to wrap lines which are too long.
    ///   * `None`: Use text's width as number of columns (maximum line length).
    /// * `trim`: Whether to trim white-spaces around the lines before aligment.
    /// * `bias`: Which side to bias towards if line can't be perfectly centered.
    /// * `keep_spaces`: Whether to keep the spaces on the right.
    ///
    /// # Note
    /// This method replaces all line endings with `\n`.
    ///
    /// # Examples
    /// ```
    /// use align_text::{Align, Bias, Where};
    /// let mut text = [
    ///     "Hello           ",
    ///     "            World!",
    ///     "   This should center-align     ",
    /// ]
    /// .join("\n");
    /// let aligned = text
    ///     .align_text(Where::Center, Some((30, false)), true, Bias::Right, true)
    ///     .unwrap();
    /// assert_eq!(
    ///     aligned,
    ///     [
    ///         "             Hello            ",
    ///         "            World!            ",
    ///         "   This should center-align   "
    ///     ]
    ///     .join("\n")
    /// );
    /// ```
    fn align_text(
        &self,
        align: Where,
        columns: Option<(usize, bool)>,
        trim: bool,
        bias: Bias,
        keep_spaces: bool,
    ) -> Result<String, Error> {
        let aligned = self
            .lines()
            .map(|line| line.to_string())
            .collect::<Vec<String>>()
            .align_text(align, columns, trim, bias, keep_spaces)?
            .join("\n");

        Ok(aligned)
    }
}

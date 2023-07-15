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
/// * [`Error::EmptyVector`]: `lines` is an empty vector.
/// * [`Error::InsufficientWidth`]: the `lines` can't fit in the given `width`.
/// * [`Error::UnknownError`]: an unexpected error that shouldn't have occured.
///
/// # Examples
/// * Passing an empty vector:
/// ```
/// use align::{Align, Where, Bias, Error};
/// let mut lines = Vec::new();
/// let result = lines.align_text(Where::Center, None, true, Bias::Right, true);
/// assert_eq!(result, Err(Error::EmptyVector));
/// ```
/// * Passing an insufficient width:
/// ```
/// use align::{Align, Where, Bias, Error};
/// let mut lines = vec!["0123456789".to_string()];
/// let result = lines.align_text(Where::Center, Some(3), true, Bias::Right, true);
/// assert_eq!(result, Err(Error::InsufficientWidth));
/// ```
#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    EmptyVector,
    InsufficientWidth,
    UnknownError(&'static str),
}

/// The trait which defines the align_text() function.
/// No defaut implementation.
/// Implemented for [`Vec<String>`].
pub trait Align {
    fn align_text(
        &mut self,
        align: Where,
        width: Option<usize>,
        trim: bool,
        bias: Bias,
        keep_spaces: bool,
    ) -> Result<(), Error>;
}

impl Align for Vec<String> {
    /// Aligns each line (String) of text within `width` columns by inserting spaces to its left and right.
    /// See [`Error`] for potential errors returned.
    /// # Params
    /// * `align`: Where to align the lines.
    /// * `width`: Final width to align in. If none, defaults to maximum line length.
    /// * `trim`: Whether to trim white-spaces around the lines before aligment.
    /// * `bias`: Which side to bias towards if line can't be perfectly centered.
    /// * `keep_spaces`: Whether to keep the spaces on the right.
    ///
    /// # Examples
    /// ```
    /// use align::{Align, Where, Bias};
    ///
    /// let mut lines = vec![
    ///     "Hello           ".to_string(),
    ///     "            World!".to_string(),
    ///     "   This should justify center     ".to_string(),
    /// ];
    /// lines.align_text(Where::Center, Some(30), true, Bias::Right, true).unwrap();
    ///
    /// assert_eq!(lines[0], "             Hello            ");
    /// assert_eq!(lines[1], "            World!            ");
    /// assert_eq!(lines[2], "  This should justify center  ");
    /// ```
    fn align_text(
        &mut self,
        align: Where,
        width: Option<usize>,
        trim: bool,
        bias: Bias,
        keep_spaces: bool,
    ) -> Result<(), Error> {
        let mut lines = self.clone();

        if lines.len() == 0 {
            return Err(Error::EmptyVector);
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

        let width = match width {
            Some(w) if w < text_width => return Err(Error::InsufficientWidth),
            Some(w) => w,
            None => text_width,
        };

        // align by adding spaces before and after
        for line in lines.iter_mut() {
            let space = width - line.len();

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

        *self = lines;
        Ok(())
    }
}

pub mod align {
    #[derive(Debug)]
    pub enum Where {
        Left,
        Center,
        Right,
    }

    #[derive(Debug, Clone, Copy)]
    pub enum Bias {
        Left,
        Right,
    }

    impl From<Bias> for usize {
        fn from(value: Bias) -> Self {
            match value {
                Bias::Left => 0,
                Bias::Right => 1,
            }
        }
    }

    /// Errors returned by justify_text() and align_text()
    /// * `UnknownError`: an unexpected error that shouldn't have occured.
    /// * `EmptyVector`: the function received an empty vector as the `lines` param.
    /// * `InsufficientWidth`: the text can't fit in the given `width`.
    #[derive(Debug)]
    pub enum Error {
        UnknownError(&'static str),
        EmptyVector,
        InsufficientWidth,
    }

    /// Justifies the text and returns
    /// * An `Ok()` containing the width of the text.
    /// * An `Err()` containing an [`Error`] encountered during execution.
    /// # Params
    /// * `lines`: lines of text to justify.
    /// * `justify`: where to justify the text.
    /// * `trim`: whether to trim the spaces around the lines before justifying.
    /// * `bias`: which side the extra space should be on, if the line can't be centered perfectly.
    /// ### bias details
    /// Only matters if `justify` is [`Where::Center`].
    ///
    /// If a line's length is even and the text's width is odd, or vice-versa, then the line can't be centered perfectly.
    /// So an extra space is added either to the left or the right of the line.  
    /// This param controls on which side the extra space will be added, making the line closer to the `bias` side.
    ///
    /// # Panics
    /// If `lines` is and empty vector.
    ///
    /// # Examples
    /// ```
    /// use align::align::{justify_text, Where, Bias};
    ///
    /// let mut lines = vec![
    ///     "Hello           ".to_string(),
    ///     "            World!".to_string(),
    ///     "   This should justify center     ".to_string(),
    /// ];
    /// justify_text(&mut lines, Where::Center, Some(30), true, Bias::Right, true).unwrap();
    ///
    /// assert_eq!(lines[0], "             Hello            ");
    /// assert_eq!(lines[1], "            World!            ");
    /// assert_eq!(lines[2], "  This should justify center  ");
    /// ```
    pub fn justify_text(
        lines: &mut Vec<String>,
        align: Where,
        width: Option<usize>,
        trim: bool,
        bias: Bias,
        keep_spaces: bool,
    ) -> Result<(), Error> {
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

        // add spaces before and after each line
        for line in lines {
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
        };

        Ok(())
    }
}

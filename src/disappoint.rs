use std::{any::Any, fmt};

/// A set of `Disappoint`s occurred during an execution of `expected`.
#[derive(Debug)]
pub struct Disappoints(pub(crate) Vec<Disappoint>);

impl std::ops::Deref for Disappoints {
    type Target = [Disappoint];

    #[inline]
    fn deref(&self) -> &Self::Target {
        &*self.0
    }
}

impl fmt::Display for Disappoints {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "one or more expectations have been disappointed:")?;
        for disappoint in &self.0 {
            writeln!(f, "{}", disappoint)?;
        }
        Ok(())
    }
}

/// A struct providing information about a disappointing expectation.
#[derive(Debug)]
pub struct Disappoint {
    payload: Box<dyn Any + Send + 'static>,
    file: &'static str,
    line: u32,
    column: u32,
}

impl fmt::Display for Disappoint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let payload = self.payload_str().unwrap_or("Box<dyn Any>");
        writeln!(
            f,
            "[{}:{}:{}] {}",
            self.file, self.line, self.column, payload,
        )
    }
}

impl Disappoint {
    #[inline]
    pub(crate) fn new(
        payload: Box<dyn Any + Send>,
        file: &'static str,
        line: u32,
        column: u32,
    ) -> Self {
        Self {
            payload,
            file,
            line,
            column,
        }
    }

    pub(crate) fn payload_str(&self) -> Option<&str> {
        let payload = self.payload();
        (payload.downcast_ref::<&str>().copied())
            .or_else(|| payload.downcast_ref::<String>().map(|s| s.as_str()))
    }

    /// Return the payload associated with the disappointment.
    ///
    /// The concrete type of payloads is commonly either of `&'static str`
    /// or `String`, but it may change in the future version.
    #[inline]
    pub fn payload(&self) -> &(dyn Any + Send + 'static) {
        &*self.payload
    }

    /// Return the name of the source file from which the disappointment originated.
    #[inline]
    pub fn file(&self) -> &str {
        self.file
    }

    /// Return the line number from which the disappointment originated.
    #[inline]
    pub fn line(&self) -> u32 {
        self.line
    }

    /// Return the column from which the disappointment originated.
    #[inline]
    pub fn column(&self) -> u32 {
        self.column
    }
}

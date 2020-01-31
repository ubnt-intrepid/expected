use std::fmt;

/// A set of disappoints occurred during an execution of `asseverate`.
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

#[derive(Debug)]
pub struct Disappoint {
    payload: String,
    file: &'static str,
    line: u32,
    column: u32,
}

impl fmt::Display for Disappoint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "[{}:{}:{}] {}",
            self.file, self.line, self.column, self.payload,
        )
    }
}

impl Disappoint {
    #[inline]
    pub(crate) fn new(payload: String, file: &'static str, line: u32, column: u32) -> Self {
        Self {
            payload,
            file,
            line,
            column,
        }
    }

    #[inline]
    pub fn payload(&self) -> &str {
        &*self.payload
    }

    #[inline]
    pub fn file(&self) -> &str {
        self.file
    }

    #[inline]
    pub fn line(&self) -> u32 {
        self.line
    }

    #[inline]
    pub fn column(&self) -> u32 {
        self.column
    }
}

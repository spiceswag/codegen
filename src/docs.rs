use std::fmt::{self, Write};

use crate::formatter::Formatter;

/// Printable documentation that auto inserts leading triple slashes for each line.
#[derive(Debug, Clone)]
pub struct Docs {
    docs: String,
    inner: bool,
}

impl Docs {
    pub fn new(docs: &str) -> Self {
        Docs {
            docs: docs.to_string(),
            inner: false,
        }
    }

    pub fn new_inner(docs: &str) -> Self {
        Docs {
            docs: docs.to_string(),
            inner: true,
        }
    }

    pub fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        if !self.inner {
            for line in self.docs.lines() {
                write!(fmt, "/// {}\n", line)?;
            }
        } else {
            for line in self.docs.lines() {
                write!(fmt, "//! {}\n", line)?;
            }
        }

        Ok(())
    }
}

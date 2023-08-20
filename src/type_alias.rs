use std::fmt::{self, Write};

use crate::{type_def::TypeDef, Formatter, Type};

/// Define a type alias.
#[derive(Debug, Clone)]
pub struct TypeAlias {
    type_def: TypeDef,
    actual_type: Type,
}

impl TypeAlias {
    /// Return a type alias with the given name and actual type.
    pub fn new(name: &str, actual_type: Type) -> Self {
        Self {
            type_def: TypeDef::new(name),
            actual_type,
        }
    }

    /// Set the type alias' documentation
    pub fn doc(&mut self, doc: &str) -> &mut Self {
        self.type_def.doc(doc);
        self
    }

    /// Add a `where` bound to the type alias.
    ///
    /// Do note that these are as of the current version of rustc not enforced.
    pub fn bound<T: Into<Type>>(&mut self, name: &str, ty: T) -> &mut Self {
        self.type_def.bound(name, ty);
        self
    }

    /// Formats the type alias using the given formatter.
    pub fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        self.type_def.fmt_head("type", &[], fmt)?;
        write!(fmt, " = ")?;
        self.actual_type.fmt(fmt)?;
        write!(fmt, ";")?;

        Ok(())
    }
}

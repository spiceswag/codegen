use std::fmt::{self, Write};

use indexmap::IndexMap;

use crate::docs::Docs;
use crate::formatter::Formatter;
use crate::function::Function;
use crate::import::Import;
use crate::item::Item;
use crate::module::Module;
use crate::{Type, TypeAlias};

use crate::r#enum::Enum;
use crate::r#impl::Impl;
use crate::r#struct::Struct;
use crate::r#trait::Trait;

/// Defines a scope.
///
/// A scope contains modules, types, etc...
#[derive(Debug, Clone)]
pub struct Scope {
    /// Scope documentation
    docs: Option<Docs>,

    /// Lint attributes to supress for the entire scope
    allow: Vec<String>,

    /// Imports
    imports: IndexMap<String, IndexMap<String, Import>>,

    /// Contents of the documentation,
    items: Vec<Item>,
}

impl Scope {
    /// Returns a new scope
    pub fn new() -> Self {
        Scope {
            docs: None,
            allow: vec![],
            imports: IndexMap::new(),
            items: vec![],
        }
    }

    /// Set the documentation for the scope.
    ///
    /// The documentation comments are formatted as
    /// inner documentation comments (`//!`).
    pub fn doc(&mut self, docs: &str) -> &mut Self {
        self.docs = Some(Docs::new_inner(docs));
        self
    }

    /// Specify a lint attribute to supress for the entire scope
    ///
    /// The allow attributes are formatted as
    /// inner attributes (`#![allow()]`)
    pub fn allow(&mut self, allow: &str) -> &mut Self {
        self.allow.push(allow.to_owned());
        self
    }

    /// Import a type into the scope.
    ///
    /// This results in a new `use` statement being added to the beginning of
    /// the scope.
    pub fn import(&mut self, path: &str, ty: &str) -> &mut Import {
        // handle cases where the caller wants to refer to a type namespaced
        // within the containing namespace, like "a::B".
        let ty = ty.split("::").next().unwrap_or(ty);
        self.imports
            .entry(path.to_string())
            .or_insert(IndexMap::new())
            .entry(ty.to_string())
            .or_insert_with(|| Import::new(path, ty))
    }

    /// Push a new module definition, returning a mutable reference to it.
    ///
    /// # Panics
    ///
    /// Since a module's name must uniquely identify it within the scope in
    /// which it is defined, pushing a module whose name is already defined
    /// in this scope will cause this function to panic.
    ///
    /// In many cases, the [`get_or_new_module`] function is preferrable, as it
    /// will return the existing definition instead.
    ///
    /// [`get_or_new_module`]: #method.get_or_new_module
    pub fn new_module(&mut self, name: &str) -> &mut Module {
        self.push_module(Module::new(name));

        match *self.items.last_mut().unwrap() {
            Item::Module(ref mut v) => v,
            _ => unreachable!(),
        }
    }

    /// Returns a mutable reference to a module if it is exists in this scope.
    pub fn get_module_mut<Q: ?Sized>(&mut self, name: &Q) -> Option<&mut Module>
    where
        String: PartialEq<Q>,
    {
        self.items.iter_mut().find_map(|item| match item {
            &mut Item::Module(ref mut module) if module.name == *name => Some(module),
            _ => None,
        })
    }

    /// Returns a mutable reference to a module if it is exists in this scope.
    pub fn get_module<Q: ?Sized>(&self, name: &Q) -> Option<&Module>
    where
        String: PartialEq<Q>,
    {
        self.items
            .iter()
            .filter_map(|item| match item {
                &Item::Module(ref module) if module.name == *name => Some(module),
                _ => None,
            })
            .next()
    }

    /// Returns a mutable reference to a module, creating it if it does
    /// not exist.
    pub fn get_or_new_module(&mut self, name: &str) -> &mut Module {
        if self.get_module(name).is_some() {
            self.get_module_mut(name).unwrap()
        } else {
            self.new_module(name)
        }
    }

    /// Push a module definition.
    ///
    /// # Panics
    ///
    /// Since a module's name must uniquely identify it within the scope in
    /// which it is defined, pushing a module whose name is already defined
    /// in this scope will cause this function to panic.
    ///
    /// In many cases, the [`get_or_new_module`] function is preferrable, as it will
    /// return the existing definition instead.
    ///
    /// [`get_or_new_module`]: #method.get_or_new_module
    pub fn push_module(&mut self, item: Module) -> &mut Self {
        assert!(self.get_module(&item.name).is_none());
        self.items.push(Item::Module(item));
        self
    }

    /// Push a new struct definition, returning a mutable reference to it.
    pub fn new_struct(&mut self, name: &str) -> &mut Struct {
        self.push_struct(Struct::new(name));

        match *self.items.last_mut().unwrap() {
            Item::Struct(ref mut v) => v,
            _ => unreachable!(),
        }
    }

    /// Push a struct definition
    pub fn push_struct(&mut self, item: Struct) -> &mut Self {
        self.items.push(Item::Struct(item));
        self
    }

    /// Push a new function definition, returning a mutable reference to it.
    pub fn new_fn(&mut self, name: &str) -> &mut Function {
        self.push_fn(Function::new(name));

        match *self.items.last_mut().unwrap() {
            Item::Function(ref mut v) => v,
            _ => unreachable!(),
        }
    }

    /// Push a function definition
    pub fn push_fn(&mut self, item: Function) -> &mut Self {
        self.items.push(Item::Function(item));
        self
    }

    /// Push a new trait definition, returning a mutable reference to it.
    pub fn new_trait(&mut self, name: &str) -> &mut Trait {
        self.push_trait(Trait::new(name));

        match *self.items.last_mut().unwrap() {
            Item::Trait(ref mut v) => v,
            _ => unreachable!(),
        }
    }

    /// Push a trait definition
    pub fn push_trait(&mut self, item: Trait) -> &mut Self {
        self.items.push(Item::Trait(item));
        self
    }

    /// Push a new struct definition, returning a mutable reference to it.
    pub fn new_enum(&mut self, name: &str) -> &mut Enum {
        self.push_enum(Enum::new(name));

        match *self.items.last_mut().unwrap() {
            Item::Enum(ref mut v) => v,
            _ => unreachable!(),
        }
    }

    /// Push a structure definition
    pub fn push_enum(&mut self, item: Enum) -> &mut Self {
        self.items.push(Item::Enum(item));
        self
    }

    /// Push a new `impl` block, returning a mutable reference to it.
    pub fn new_impl(&mut self, target: &str) -> &mut Impl {
        self.push_impl(Impl::new(target));

        match *self.items.last_mut().unwrap() {
            Item::Impl(ref mut v) => v,
            _ => unreachable!(),
        }
    }

    /// Push an `impl` block.
    pub fn push_impl(&mut self, item: Impl) -> &mut Self {
        self.items.push(Item::Impl(item));
        self
    }

    /// Push a new type alias (`type`) and return a new mutable reference to it.
    pub fn new_type_alias(&mut self, name: &str, actual_type: Type) -> &mut TypeAlias {
        self.push_type_alias(TypeAlias::new(name, actual_type));

        match *self.items.last_mut().unwrap() {
            Item::TypeAlias(ref mut v) => v,
            _ => unreachable!(),
        }
    }

    /// Push a type alias (`type`)
    pub fn push_type_alias(&mut self, item: TypeAlias) -> &mut Self {
        self.items.push(Item::TypeAlias(item));
        self
    }

    /// Push a raw string to the scope.
    ///
    /// This string will be included verbatim in the formatted string.
    pub fn raw(&mut self, val: &str) -> &mut Self {
        self.items.push(Item::Raw(val.to_string()));
        self
    }

    /// Return a string representation of the scope.
    pub fn to_string(&self) -> String {
        let mut ret = String::new();

        self.fmt(&mut Formatter::new(&mut ret)).unwrap();

        // Remove the trailing newline
        if ret.as_bytes().last() == Some(&b'\n') {
            ret.pop();
        }

        ret
    }

    /// Formats the scope using the given formatter.
    pub fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        if let Some(docs) = &self.docs {
            docs.fmt(fmt)?;
            write!(fmt, "\n")?;
        }

        self.fmt_allow(fmt)?;

        self.fmt_imports(fmt)?;

        if !self.imports.is_empty() {
            write!(fmt, "\n")?;
        }

        for (i, item) in self.items.iter().enumerate() {
            if i != 0 {
                write!(fmt, "\n")?;
            }

            match *item {
                Item::Module(ref v) => v.fmt(fmt)?,
                Item::Struct(ref v) => v.fmt(fmt)?,
                Item::Function(ref v) => v.fmt(false, fmt)?,
                Item::Trait(ref v) => v.fmt(fmt)?,
                Item::Enum(ref v) => v.fmt(fmt)?,
                Item::Impl(ref v) => v.fmt(fmt)?,
                Item::TypeAlias(ref v) => v.fmt(fmt)?,
                Item::Raw(ref v) => {
                    write!(fmt, "{}\n", v)?;
                }
            }
        }

        Ok(())
    }

    fn fmt_imports(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        // First, collect all visibilities
        let mut visibilities = vec![];

        for (_, imports) in &self.imports {
            for (_, import) in imports {
                if !visibilities.contains(&import.vis) {
                    visibilities.push(import.vis.clone());
                }
            }
        }

        let mut tys = vec![];

        // Loop over all visibilities and format the associated imports
        for vis in &visibilities {
            for (path, imports) in &self.imports {
                tys.clear();

                for (ty, import) in imports {
                    if *vis == import.vis {
                        tys.push(ty);
                    }
                }

                if !tys.is_empty() {
                    if let Some(ref vis) = *vis {
                        write!(fmt, "{} ", vis)?;
                    }

                    write!(fmt, "use {}::", path)?;

                    if tys.len() > 1 {
                        write!(fmt, "{{")?;

                        for (i, ty) in tys.iter().enumerate() {
                            if i != 0 {
                                write!(fmt, ", ")?;
                            }
                            write!(fmt, "{}", ty)?;
                        }

                        write!(fmt, "}};\n")?;
                    } else if tys.len() == 1 {
                        write!(fmt, "{};\n", tys[0])?;
                    }
                }
            }
        }

        Ok(())
    }

    // The canonical way to output more than one allow at the same time is `#![allow(a, b, c)]`
    // but to keep consistent in the library I have to output `#![allow(a)] \n #![allow(b)]`
    fn fmt_allow(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        if !self.allow.is_empty() {
            writeln!(fmt, "#![allow({})]", self.allow.join(", "))?;
        }
        Ok(())
    }
}

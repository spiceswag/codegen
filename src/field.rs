use crate::r#type::Type;

/// Defines a struct field.
#[derive(Debug, Clone)]
pub struct Field {
    /// Field name
    pub name: String,

    /// Field type
    pub ty: Type,

    /// Field visibility
    pub vis: Option<String>,

    /// Field documentation
    pub documentation: Vec<String>,

    /// Field annotation
    pub annotation: Vec<String>,
}

impl Field {
    /// Return a field definition with the provided name and type
    pub fn new<T>(name: &str, ty: T) -> Self
    where
        T: Into<Type>,
    {
        Field {
            name: name.into(),
            ty: ty.into(),
            vis: None,
            documentation: Vec::new(),
            annotation: Vec::new(),
        }
    }

    /// Set the field's visibility.
    pub fn vis(&mut self, vis: &str) -> &mut Self {
        self.vis = Some(vis.to_string());
        self
    }

    /// Set field's documentation.
    pub fn doc(&mut self, documentation: Vec<&str>) -> &mut Self {
        self.documentation = documentation.iter().map(|doc| doc.to_string()).collect();
        self
    }

    /// Set field's annotation.
    pub fn annotation(&mut self, annotation: Vec<&str>) -> &mut Self {
        self.annotation = annotation.iter().map(|ann| ann.to_string()).collect();
        self
    }
}

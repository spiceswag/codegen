/// Defines an import (`use` statement).
#[derive(Debug, Clone)]
pub struct Import {
    _line: String,

    /// Re-export visibility
    pub vis: Option<String>,
}

impl Import {
    /// Return a new import.
    pub fn new(path: &str, ty: &str) -> Self {
        Import {
            _line: format!("{}::{}", path, ty),
            vis: None,
        }
    }

    /// Set the import visibility.
    pub fn vis(&mut self, vis: &str) -> &mut Self {
        self.vis = Some(vis.to_string());
        self
    }
}

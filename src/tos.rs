use serde::{Deserialize, Serialize};

/// Represents a single stylesheet in the `.fobz` document.
///
/// # Fields
/// - `path`: Path to the stylesheet file within the `.fobz` archive.
#[derive(Debug, Serialize, Deserialize)]
pub struct StyleInfo {
    pub path: String,
}

/// Represents the table of stylesheets, managing all CSS files in the `.fobz` document.
///
/// # Fields
/// - `styles`: A vector of `StyleInfo` items, each pointing to a distinct stylesheet.
#[derive(Debug, Serialize, Deserialize)]
pub struct TableOfStyles {
    styles: Vec<StyleInfo>,
}

impl TableOfStyles {
    /// Creates a new `TableOfStyles` instance.
    ///
    /// Initializes the table of stylesheets with an empty vector.
    pub fn new() -> Self {
        TableOfStyles { styles: vec![] }
    }

    /// Retrieves a reference to the `StyleInfo` associated with the given path.
    ///
    /// # Parameters
    /// - `path`: The path of the stylesheet to search for.
    ///
    /// # Returns
    /// An `Option` containing a reference to `StyleInfo` if found, or `None` if not found.
    pub fn get(&self, path: &String) -> Option<&StyleInfo> {
        self.styles.iter().find(|v| &v.path == path)
    }

    /// Adds a new stylesheet to the table of stylesheets.
    ///
    /// # Parameters
    /// - `info`: The `StyleInfo` object representing the stylesheet to add.
    pub fn add(&mut self, info: StyleInfo) {
        self.styles.append(&mut vec![info]);
    }

    /// Removes a stylesheet from the table of stylesheets by its path.
    ///
    /// # Parameters
    /// - `path`: The path of the stylesheet to remove.
    pub fn remove(&mut self, path: &String) {
        self.styles.retain(|v| &v.path != path);
    }
}

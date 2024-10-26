use serde::{Deserialize, Serialize};

/// Represents a single section within the `.fobz` document.
///
/// # Fields
/// - `path`: Path to the section file within the `.fobz` archive.
/// - `title`: Title of the section, for display purposes.
#[derive(Debug, Serialize, Deserialize)]
pub struct ContentInfo {
    pub path: String,
    pub title: String,
}

/// Represents the table of contents for a `.fobz` document, organizing multiple sections.
///
/// # Fields
/// - `sections`: A vector of `ContentInfo` items, each representing a distinct part of the document.
#[derive(Debug, Serialize, Deserialize)]
pub struct TableOfContents {
    sections: Vec<ContentInfo>,
}

impl TableOfContents {
    /// Creates a new `TableOfContents` instance.
    ///
    /// Initializes the table of contents with an empty vector of sections.
    pub fn new() -> Self {
        TableOfContents { sections: vec![] }
    }

    /// Retrieves a reference to the `ContentInfo` associated with the given path.
    ///
    /// # Parameters
    /// - `path`: The path of the section to search for.
    ///
    /// # Returns
    /// An `Option` containing a reference to `ContentInfo` if found, or `None` if not found.
    pub fn get(&self, path: &String) -> Option<&ContentInfo> {
        self.sections.iter().find(|v| &v.path == path)
    }

    /// Adds a new section to the table of contents.
    ///
    /// # Parameters
    /// - `info`: The `ContentInfo` object representing the section to add.
    pub fn add(&mut self, info: ContentInfo) {
        self.sections.append(&mut vec![info]);
    }

    /// Removes a section from the table of contents by its path.
    ///
    /// # Parameters
    /// - `path`: The path of the section to remove.
    pub fn remove(&mut self, path: &String) {
        self.sections.retain(|v| &v.path != path);
    }
}

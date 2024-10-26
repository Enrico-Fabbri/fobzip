use serde::{Deserialize, Serialize};

/// Represents a single resource used in a `.fobz` document (e.g., images).
///
/// # Fields
/// - `path`: Path to the resource file within the `.fobz` archive.
/// - `name`: Descriptive name of the resource used if unable to load the file.
#[derive(Debug, Serialize, Deserialize)]
pub struct ResourceInfo {
    pub path: String,
    pub name: String,
}

/// Represents the table of resources, a collection of resources used in the `.fobz` document.
///
/// # Fields
/// - `resources`: A vector of `ResourceInfo` items.
#[derive(Debug, Serialize, Deserialize)]
pub struct TableOfResources {
    resources: Vec<ResourceInfo>,
}

impl TableOfResources {
    /// Creates a new `TableOfResources` instance.
    ///
    /// Initializes the table of resources with an empty vector.
    pub fn new() -> Self {
        TableOfResources { resources: vec![] }
    }

    /// Retrieves a reference to the `ResourceInfo` associated with the given path.
    ///
    /// # Parameters
    /// - `path`: The path of the resource to search for.
    ///
    /// # Returns
    /// An `Option` containing a reference to `ResourceInfo` if found, or `None` if not found.
    pub fn get(&self, path: &String) -> Option<&ResourceInfo> {
        self.resources.iter().find(|v| &v.path == path)
    }

    /// Adds a new resource to the table of resources.
    ///
    /// # Parameters
    /// - `info`: The `ResourceInfo` object representing the resource to add.
    pub fn add(&mut self, info: ResourceInfo) {
        self.resources.append(&mut vec![info]);
    }

    /// Removes a resource from the table of resources by its path.
    ///
    /// # Parameters
    /// - `path`: The path of the resource to remove.
    pub fn remove(&mut self, path: &String) {
        self.resources.retain(|v| &v.path != path);
    }
}

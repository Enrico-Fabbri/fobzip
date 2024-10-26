use serde::{Deserialize, Serialize};

/// Represents the document's metadata in the `.fobz` format.
///
/// # Fields
/// - `version`: The version of the document format (e.g., "1.0").
/// - `title`: The title of the document.
/// - `author`: The name of the author of the document.
/// - `description`: A brief summary or description of the document.
/// - `tags`: A list of tags classifying the document's genre or themes.
/// - `index`: The relative path of the starting page.
/// - `cover`: The relative path of the cover image.
#[derive(Debug, Serialize, Deserialize)]
pub struct Manifest {
    version: String,
    title: String,
    author: String,
    description: String,
    tags: Vec<String>,
    index: String,
    cover: String,
}

impl Default for Manifest {
    /// Creates a default instance of `Manifest`.
    ///
    /// The default values are:
    /// - `version`: "1.0"
    /// - `title`: ""
    /// - `author`: ""
    /// - `description`: ""
    /// - `tags`: []
    /// - `index`: "default/no_section.html"
    /// - `cover`: "default/no_cover.jpg"
    fn default() -> Self {
        Self {
            version: "1.0".into(),
            title: "".into(),
            author: "".into(),
            description: "".into(),
            tags: vec![],
            index: "default/no_section.html".into(),
            cover: "default/no_cover.jpg".into(),
        }
    }
}

impl Manifest {
    /// Creates a new `Manifest` instance with the provided title, author, description, and tags.
    ///
    /// # Parameters
    /// - `title`: The title of the document.
    /// - `author`: The name of the author of the document.
    /// - `description`: A brief summary or description of the document.
    /// - `tags`: A list of tags classifying the document's genre or themes.
    ///
    /// # Returns
    /// A new `Manifest` instance initialized with the provided values and default values for other fields.
    pub fn new(title: String, author: String, description: String, tags: Vec<String>) -> Self {
        Manifest {
            version: "1.0".into(),
            title,
            author,
            description,
            tags,
            index: "default/no_section.html".into(),
            cover: "default/no_cover.jpg".into(),
        }
    }

    /// Retrieves a mutable reference to the document's version.
    ///
    /// # Returns
    /// A mutable reference to the version string.
    pub fn get_version(&mut self) -> &String {
        &mut self.version
    }

    /// Retrieves a mutable reference to the document's title.
    ///
    /// # Returns
    /// A mutable reference to the title string.
    pub fn get_title(&mut self) -> &String {
        &mut self.title
    }

    /// Retrieves a mutable reference to the document's author.
    ///
    /// # Returns
    /// A mutable reference to the author string.
    pub fn get_author(&mut self) -> &String {
        &mut self.author
    }

    /// Retrieves a mutable reference to the document's description.
    ///
    /// # Returns
    /// A mutable reference to the description string.
    pub fn get_description(&mut self) -> &String {
        &mut self.description
    }

    /// Retrieves a mutable reference to the list of tags associated with the document.
    ///
    /// # Returns
    /// A mutable reference to the vector of tags.
    pub fn get_tags(&mut self) -> &Vec<String> {
        &mut self.tags
    }

    /// Retrieves a mutable reference to the document's index path.
    ///
    /// # Returns
    /// A mutable reference to the index path string.
    pub fn get_index(&mut self) -> &String {
        &mut self.index
    }

    /// Retrieves a mutable reference to the document's cover image path.
    ///
    /// # Returns
    /// A mutable reference to the cover image path string.
    pub fn get_cover(&mut self) -> &String {
        &mut self.cover
    }

    /// Sets the title of the document.
    ///
    /// # Parameters
    /// - `title`: The new title of the document.
    pub fn set_title(&mut self, title: String) {
        self.title = title;
    }

    /// Sets the author of the document.
    ///
    /// # Parameters
    /// - `author`: The new author of the document.
    pub fn set_author(&mut self, author: String) {
        self.author = author;
    }

    /// Sets the description of the document.
    ///
    /// # Parameters
    /// - `description`: The new description of the document.
    pub fn set_description(&mut self, description: String) {
        self.description = description;
    }

    /// Adds new tags to the document.
    ///
    /// # Parameters
    /// - `tags`: A vector of tags to add to the document.
    pub fn add_tags(&mut self, mut tags: Vec<String>) {
        self.tags.append(&mut tags);
    }

    /// Removes specified tags from the document.
    ///
    /// # Parameters
    /// - `tags`: A vector of tags to remove from the document.
    pub fn remove_tags(&mut self, tags: Vec<String>) {
        self.tags.retain(|v| !tags.contains(v));
    }

    /// Sets the relative path of the starting page for the document.
    ///
    /// # Parameters
    /// - `path`: The new path for the starting page.
    pub fn set_index(&mut self, path: String) {
        self.index = path;
    }

    /// Sets the relative path of the cover image for the document.
    ///
    /// # Parameters
    /// - `path`: The new path for the cover image.
    pub fn set_cover(&mut self, path: String) {
        self.cover = path;
    }
}

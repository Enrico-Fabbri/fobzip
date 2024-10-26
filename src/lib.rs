//! # `.fobz` File Format
//!
//! A `.fobz` file is a ZIP-based archive format designed to organize text contents and resources
//! for easy manipulation and management. Below is the structure and explanation of its components:
//!
//! ## File Structure
//!
//! A `.fobz` file contains the following key components:
//!
//! - `manifest.json`
//! - `toc.json`
//! - `tor.json`
//! - `tos.json`
//! - `contents/`
//! - `resources/`
//! - `styles/`
//!
//! ### `manifest.json`
//!
//! The `manifest.json` file contains metadata about the document. The fields are as follows:
//!
//! ```json
//! {
//!     "version": "1.0",
//!     "title": "Sample Document",
//!     "author": "John Doe",
//!     "description": "A short description of the document.",
//!     "tags": [
//!         "fiction",
//!         "adventure",
//!         "sample"
//!     ],
//!     "index": "contents/introduction.html"
//!     "cover": "resource1",
//! }
//! ```
//!
//! - `version`: A string specifying the version of the document format.
//! - `title`: A string representing the title of the document.
//! - `author`: The name of the author.
//! - `description`: A brief description or synopsis of the document.
//! - `tags`: An array containing the tags categorizing the document.
//! - `index`: The starting point of the document.
//! - `cover`: The cover image of the document.
//!
//! ### `toc.json` (Table of Contents)
//!
//! This file lists the contents in the document. Each section is defined by a `Section` object:
//!
//! ```json
//! {
//!     "contents": [
//!         {
//!             "path": "contents/introduction.html"
//!             "title": "Introduction",
//!         },
//!         {
//!             "path": "contents/chapter1.html"
//!             "title": "Chapter 1",
//!         }
//!     ]
//! }
//! ```
//!
//! - `path`: The file path where the section content is stored.
//! - `title`: The title of the section.
//!
//! ### `tor.json` (Table of Resources)
//!
//! This file lists the resources (e.g., images) used in the document. Each resource is represented by a `Resource` object:
//!
//! ```json
//! {
//!     "resources": [
//!         {
//!             "path": "resources/cover.png"
//!             "name": "Cover Image",
//!         },
//!         {
//!             "path": "resources/image.jpg"
//!             "name": "Cat Image",
//!         }
//!     ]
//! }
//! ```
//!
//! - `path`: The file path to the resource.
//! - `name`: A human-readable name for the resource.
//!
//! ### `tos.json` (Table of Styles)
//!
//! This file lists the styles used in the document. Each style is represented by a `Style` object:
//!
//! ```json
//! {
//!     "styles": [
//!         {
//!             "path": "styles/main.css"
//!         },
//!         {
//!             "path": "styles/chapter.css"
//!         }
//!     ]
//! }
//! ```
//!
//! - `path`: The file path to the style.
//!
//! ### `contents/` Directory
//!
//! This directory contains the actual content of the document. Each section is stored as a separate file in HTML format.
//!
//! ### `resources/` Directory
//!
//! This directory stores additional resources like images. Each resource is referenced in `tor.json` and stored as a file inside this directory.
//!
//! ### `styles/` Directory
//!
//! Stores CSS stylesheets that apply styling to the document. Each stylesheet is referenced in `tos.json`
//! and can be linked dynamically to contents as needed.
//!
//! ## Example File Structure
//!
//! ```plaintext
//! my_document.fobz
//! │
//! ├── manifest.json            // Metadata of the document
//! ├── toc.json                 // Table of Contents
//! ├── tor.json                 // Table of Resources
//! ├── tos.json                 // Table of Styles
//! ├── contents/                // Directory containing document contents
//! │   ├── introduction.html    
//! │   ├── chapter1.html        
//! │   └── ...
//! ├── resources/               // Directory containing additional resources
//! │   ├── cover.png            
//! |   ├── video.mp4            
//! |   └── ...
//! ├── styles/                  // Directory containing CSS stylesheets
//! |   ├── main.css             
//! |   ├── chapter.css          
//! |   └── ...
//! └── default/                 // Directory containing default values
//!     ├── no_section.html         
//!     └── no_cover.jpg         
//! ```

use std::{
    collections::HashMap,
    fs::File,
    io::{Read, Write},
};

use manifest::Manifest;
use toc::{ContentInfo, TableOfContents};
use tor::{ResourceInfo, TableOfResources};
use tos::{StyleInfo, TableOfStyles};
use zip::{
    write::{ExtendedFileOptions, FileOptions},
    ZipArchive, ZipWriter,
};

/// Module handling the manifest containing the metadata.
pub mod manifest;
/// Module handling the table of contents for document contents.
pub mod toc;
/// Module for managing the table of resources (e.g., images).
pub mod tor;
/// Module dedicated to managing stylesheets used by the document.
pub mod tos;

// Constants representing default resources included in the library.
const NO_COVER: &[u8] = include_bytes!("../default/no_cover.jpg"); // Default cover image
const NO_SECTION: &str = include_str!("../default/no_section.html"); // Default section HTML

/// Represents a `.fobz` document, which includes metadata, contents, resources, and styles.
///
/// # Fields
/// - `manifest`: Metadata of the document (title, author, description, etc.).
/// - `toc`: Table of contents for the document, organizing sections.
/// - `tor`: Table of resources used in the document (e.g., images).
/// - `tos`: Table of stylesheets used in the document.
/// - `contents`: A hashmap storing the contents (HTML) of the document sections.
/// - `resources`: A hashmap storing binary resources (e.g., images).
/// - `styles`: A hashmap storing the styles (CSS) for the document.
#[derive(Debug)]
pub struct FobZ {
    manifest: Manifest,
    toc: TableOfContents,
    tor: TableOfResources,
    tos: TableOfStyles,
    contents: HashMap<String, String>,
    resources: HashMap<String, Vec<u8>>,
    styles: HashMap<String, String>,
}

impl FobZ {
    /// Creates a new `FobZ` instance with the provided title, author, description, and tags.
    ///
    /// # Parameters
    /// - `title`: The title of the document.
    /// - `author`: The author of the document.
    /// - `description`: A description of the document.
    /// - `tags`: A list of tags associated with the document.
    ///
    /// # Returns
    /// A new `FobZ` instance initialized with the provided metadata and default values for other fields.
    pub fn new(title: String, author: String, description: String, tags: Vec<String>) -> Self {
        FobZ {
            manifest: Manifest::new(title, author, description, tags),
            toc: TableOfContents::new(),
            tor: TableOfResources::new(),
            tos: TableOfStyles::new(),
            contents: HashMap::from([("default/no_section.html".into(), NO_SECTION.into())]),
            resources: HashMap::from([("default/no_cover.jpg".into(), NO_COVER.to_vec())]),
            styles: HashMap::new(),
        }
    }

    /// Opens an existing `.fobz` file and reads its contents into a `FobZ` instance.
    ///
    /// # Parameters
    /// - `path`: The file path to the `.fobz` archive.
    ///
    /// # Returns
    /// A result containing the `FobZ` instance if successful, or an error if any issue occurs.
    pub fn open(path: &str) -> anyhow::Result<Self> {
        let file = File::open(&path)?;
        let mut archive = ZipArchive::new(file)?;

        // Deserialize the JSON files in the archive into their respective structs.
        let manifest: Manifest = serde_json::from_reader(archive.by_name("manifest.json")?)?;
        let toc: TableOfContents = serde_json::from_reader(archive.by_name("toc.json")?)?;
        let tor: TableOfResources = serde_json::from_reader(archive.by_name("tor.json")?)?;
        let tos: TableOfStyles = serde_json::from_reader(archive.by_name("tos.json")?)?;

        let mut contents = HashMap::new();
        let mut resources = HashMap::new();
        let mut styles = HashMap::new();

        // Read the files in the archive and categorize them into contents, resources, and styles.
        for i in 0..archive.len() {
            let mut file = archive.by_index(i)?;
            let file_name = file.name().to_string();

            if file_name.starts_with("contents/") && file_name.ends_with(".html")
                || file_name.ends_with(".xhtml")
            {
                let mut content = String::new();
                file.read_to_string(&mut content)?;
                contents.insert(file_name, content);
            } else if file_name.starts_with("resources/")
                && (file_name.ends_with(".jpg") || file_name.ends_with(".png"))
            {
                let mut resource = Vec::new();
                file.read_to_end(&mut resource)?;
                resources.insert(file_name, resource);
            } else if file_name.starts_with("styles/") && file_name.ends_with(".css") {
                let mut style = String::new();
                file.read_to_string(&mut style)?;
                styles.insert(file_name, style);
            }
        }

        Ok(FobZ {
            manifest,
            toc,
            tor,
            tos,
            contents,
            resources,
            styles,
        })
    }

    /// Saves the current `FobZ` instance to a specified file path as a `.fobz` archive.
    ///
    /// # Parameters
    /// - `path`: The file path to save the `.fobz` archive.
    ///
    /// # Returns
    /// A result indicating success or an error if any issue occurs during saving.
    pub fn save_to(&self, path: &str) -> anyhow::Result<()> {
        let path = if path.ends_with(".fobz") {
            path.into()
        } else {
            format!("{}.fobz", path)
        };

        let file = File::create(path)?;
        let mut zip = ZipWriter::new(file);

        let options: FileOptions<'_, ExtendedFileOptions> =
            FileOptions::default().compression_method(zip::CompressionMethod::Deflated);

        // Write the metadata files to the archive.
        zip.start_file("manifest.json", options.clone())?;
        zip.write_all(serde_json::to_string_pretty(&self.manifest)?.as_bytes())?;

        zip.start_file("toc.json", options.clone())?;
        zip.write_all(serde_json::to_string_pretty(&self.toc)?.as_bytes())?;

        zip.start_file("tor.json", options.clone())?;
        zip.write_all(serde_json::to_string_pretty(&self.tor)?.as_bytes())?;

        zip.start_file("tos.json", options.clone())?;
        zip.write_all(serde_json::to_string_pretty(&self.tos)?.as_bytes())?;

        // Create directories in the archive.
        zip.add_directory("contents", options.clone())?;
        zip.add_directory("resources", options.clone())?;
        zip.add_directory("styles", options.clone())?;
        zip.add_directory("default", options.clone())?;

        // Write content files to the archive.
        for (path, content) in self.contents.iter() {
            zip.start_file(path, options.clone())?;
            zip.write_all(content.as_bytes())?;
        }

        // Write resource files to the archive.
        for (path, resource) in self.resources.iter() {
            zip.start_file(path, options.clone())?;
            zip.write_all(resource)?;
        }

        // Write style files to the archive.
        for (path, style) in self.styles.iter() {
            zip.start_file(path, options.clone())?;
            zip.write_all(style.as_bytes())?;
        }

        zip.finish()?;
        Ok(())
    }
}

impl FobZ {
    /// Adds a new content section to the document.
    ///
    /// # Parameters
    /// - `path`: The file path of the content (must end with `.html`).
    /// - `title`: The title of the content section.
    /// - `content`: The HTML content of the section.
    pub fn add_content(&mut self, path: String, title: String, content: String) {
        if !path.ends_with(".html") {
            return;
        }

        self.contents.insert(path.clone(), content);
        self.toc.add(ContentInfo { path, title });
    }

    /// Removes a content section from the document.
    ///
    /// # Parameters
    /// - `path`: The file path of the content section to remove.
    pub fn remove_content(&mut self, path: String) {
        self.contents.remove_entry(&path);
        self.toc.remove(&path);
    }

    /// Adds a new resource to the document.
    ///
    /// # Parameters
    /// - `path`: The file path of the resource (must end with `.jpg` or `.png`).
    /// - `name`: The descriptive name of the resource.
    /// - `resource`: The binary data of the resource.
    pub fn add_resource(&mut self, path: String, name: String, resource: Vec<u8>) {
        if !path.ends_with(".jpg") && !path.ends_with(".png") {
            return;
        }

        self.resources.insert(path.clone(), resource);
        self.tor.add(ResourceInfo { path, name });
    }

    /// Removes a resource from the document.
    ///
    /// # Parameters
    /// - `path`: The file path of the resource to remove.
    pub fn remove_resource(&mut self, path: String) {
        self.resources.remove_entry(&path);
        self.tor.remove(&path);
    }

    /// Adds a new stylesheet to the document.
    ///
    /// # Parameters
    /// - `path`: The file path of the stylesheet (must end with `.css`).
    /// - `style`: The CSS content of the stylesheet.
    pub fn add_style(&mut self, path: String, style: String) {
        if !path.ends_with(".css") {
            return;
        }

        self.styles.insert(path.clone(), style);
        self.tos.add(StyleInfo { path });
    }

    /// Removes a stylesheet from the document.
    ///
    /// # Parameters
    /// - `path`: The file path of the stylesheet to remove.
    pub fn remove_style(&mut self, path: String) {
        self.styles.remove_entry(&path);
        self.tos.remove(&path);
    }
}

impl FobZ {
    /// Retrieves a reference to the document's manifest.
    ///
    /// # Returns
    /// A reference to the `Manifest`.
    pub fn get_manifest(&self) -> &Manifest {
        &self.manifest
    }

    /// Retrieves information about a specific content section.
    ///
    /// # Parameters
    /// - `path`: The file path of the content section.
    ///
    /// # Returns
    /// An optional reference to `ContentInfo` if found, otherwise `None`.
    pub fn get_content_info(&self, path: &String) -> Option<&ContentInfo> {
        self.toc.get(path)
    }

    /// Retrieves both the `ContentInfo` and the content string for a specific path.
    ///
    /// # Parameters
    /// - `path`: The file path of the content section.
    ///
    /// # Returns
    /// An optional tuple containing `ContentInfo` and content string if found, otherwise `None`.
    pub fn get_content(&self, path: &String) -> Option<(&ContentInfo, &String)> {
        match self.get_content_info(&path) {
            Some(content_info) => match self.contents.get(path) {
                Some(content) => Some((content_info, content)),
                None => None,
            },
            None => None,
        }
    }

    /// Retrieves information about a specific resource.
    ///
    /// # Parameters
    /// - `path`: The file path of the resource.
    ///
    /// # Returns
    /// An optional reference to `ResourceInfo` if found, otherwise `None`.
    pub fn get_resource_info(&self, path: &String) -> Option<&ResourceInfo> {
        self.tor.get(&path)
    }

    /// Retrieves both the `ResourceInfo` and the resource data for a specific path.
    ///
    /// # Parameters
    /// - `path`: The file path of the resource.
    ///
    /// # Returns
    /// An optional tuple containing `ResourceInfo` and resource data if found, otherwise `None`.
    pub fn get_resource(&self, path: &String) -> Option<(&ResourceInfo, &Vec<u8>)> {
        match self.get_resource_info(&path) {
            Some(content_info) => match self.resources.get(path) {
                Some(resource) => Some((content_info, resource)),
                None => None,
            },
            None => None,
        }
    }

    /// Retrieves information about a specific stylesheet.
    ///
    /// # Parameters
    /// - `path`: The file path of the stylesheet.
    ///
    /// # Returns
    /// An optional reference to `StyleInfo` if found, otherwise `None`.
    pub fn get_style_info(&self, path: &String) -> Option<&StyleInfo> {
        self.tos.get(&path)
    }

    /// Retrieves both the `StyleInfo` and the stylesheet content for a specific path.
    ///
    /// # Parameters
    /// - `path`: The file path of the stylesheet.
    ///
    /// # Returns
    /// An optional tuple containing `StyleInfo` and stylesheet content if found, otherwise `None`.
    pub fn get_style(&self, path: &String) -> Option<(&StyleInfo, &String)> {
        match self.get_style_info(&path) {
            Some(content_info) => match self.styles.get(path) {
                Some(content) => Some((content_info, content)),
                None => None,
            },
            None => None,
        }
    }
}

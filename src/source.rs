use std::fs;
use std::io::BufReader;
use std::io::Read;
use std::path::PathBuf;
use std::sync::Arc;

use crate::hash::ContentHasher;
use crate::hash::FxHasher;

pub const DEFAULT_NAME: &str = "<unknown>";

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum SourceKind {
    /// A definition is a piece of code that is not executed, but can be used
    /// to define foreign symbols ( e.g from PHP ).
    Definition,

    /// A script is a piece of code that is executed.
    Script,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Source {
    pub kind: SourceKind,
    pub root: Option<PathBuf>,
    pub origin: Option<String>,
    pub content: Option<Arc<String>>,
    hasher: FxHasher,
}

/// A source.
///
/// A source is a reference to a piece of code.
///
/// Example:
///
/// ```rust
/// use ara_source::source::Source;
/// use ara_source::source::SourceKind;
///
/// let source = Source::new(SourceKind::Script, "/Documents/Project", "src/main.ara");
///
/// assert_eq!(source.kind, SourceKind::Script);
/// assert_eq!(source.origin, Some("src/main.ara".to_string()));
/// assert_eq!(source.root, Some("/Documents/Project".into()));
/// assert_eq!(source.content, None);
///
/// assert_eq!(source.name(), "src/main.ara");
/// ```
impl Source {
    /// Create a new source with the given origin.
    pub fn new<O: Into<String>, R: Into<PathBuf>>(kind: SourceKind, root: R, origin: O) -> Source {
        Source {
            kind,
            root: Some(root.into()),
            origin: Some(origin.into()),
            content: None,
            hasher: FxHasher::new(),
        }
    }

    /// Create a new source with the given content.
    ///
    /// Example:
    ///
    /// ```rust
    /// use ara_source::source::Source;
    /// use ara_source::source::SourceKind;
    ///
    /// let source = Source::inline(SourceKind::Definition, "function main(): void {}");
    ///
    /// assert_eq!(source.kind, SourceKind::Definition);
    /// assert_eq!(source.root, None);
    /// assert_eq!(source.origin, None);
    /// assert_eq!(source.content.as_ref().unwrap().as_str(), "function main(): void {}");
    ///
    /// assert_eq!(source.name(), "<unknown>");
    /// ```
    pub fn inline<C: Into<String>>(kind: SourceKind, content: C) -> Source {
        Source {
            kind,
            root: None,
            origin: None,
            content: Some(Arc::new(content.into())),
            hasher: FxHasher::new(),
        }
    }

    /// Get the name of the source.
    ///
    /// If the source has an origin, the origin is returned.
    /// Otherwise, the default name is returned.
    ///
    /// Example:
    ///
    /// ```rust
    /// use ara_source::source::Source;
    /// use ara_source::source::SourceKind;
    ///
    /// let source = Source::new(SourceKind::Definition, "/Documents/Project", "src/Foo/main.ara");
    /// assert_eq!(source.name(), "src/Foo/main.ara");
    ///
    /// let source = Source::inline(SourceKind::Definition, "function main(): void {}");
    /// assert_eq!(source.name(), "<unknown>");
    /// ```
    pub fn name(&self) -> &str {
        match &self.origin {
            Some(origin) => origin,
            None => DEFAULT_NAME,
        }
    }

    /// Returns the complete path of the source.
    ///
    /// Example:
    ///
    /// ```rust
    /// use ara_source::source::Source;
    /// use ara_source::source::SourceKind;
    ///
    /// let source = Source::new(SourceKind::Definition, "/Documents/Project", "src/Foo/main.ara");
    /// assert_eq!(source.source_path(), Some("/Documents/Project/src/Foo/main.ara".into()));
    /// ```
    pub fn source_path(&self) -> Option<PathBuf> {
        self.root
            .as_ref()
            .map(|root| root.join(self.origin.as_ref().unwrap()))
    }

    /// Returns the content of the source.
    /// If the source has no content, the content is read from the file system.
    pub fn content(&mut self) -> std::io::Result<Arc<String>> {
        if let Some(content) = self.content.as_ref() {
            return Ok(content.clone());
        }

        let path = self
            .source_path()
            .expect("Both root and origin must be present in order to read the source content");

        let mut reader = BufReader::new(fs::File::open(path)?);
        let mut file_contents = String::new();
        reader.read_to_string(&mut file_contents)?;

        let content_reference = Arc::new(file_contents);
        self.content = Some(content_reference.clone());

        Ok(content_reference)
    }

    /// Returns the hash of the source content.
    pub fn hash(&mut self) -> std::io::Result<u64> {
        let content = self.content()?;

        Ok(self.hasher.hash(&content))
    }

    /// Dispose the content of the source.
    pub fn dispose_content(&mut self) {
        self.content = None;
    }
}

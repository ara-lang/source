pub const DEFAULT_NAME: &str = "<unknown>";

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum SourceKind {
    /// A definition is a piece of code that is not executed, but can be used
    /// to define foriegn symbols ( e.g from PHP ).
    Definition,

    /// A script is a piece of code that is executed.
    Script,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Source {
    pub kind: SourceKind,
    pub origin: Option<String>,
    pub content: String,
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
/// let source = Source::new(SourceKind::Script, "main.ara", "function main(): void {}");
///
/// assert_eq!(source.kind, SourceKind::Script);
/// assert_eq!(source.origin, Some("main.ara".to_string()));
/// assert_eq!(source.content, "function main(): void {}");
///
/// assert_eq!(source.name(), "main.ara");
/// ```
impl Source {
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
    /// assert_eq!(source.origin, None);
    /// assert_eq!(source.content, "function main(): void {}");
    ///
    /// assert_eq!(source.name(), "<unknown>");
    /// ```
    pub fn new<O: Into<String>, C: Into<String>>(
        kind: SourceKind,
        origin: O,
        content: C,
    ) -> Source {
        Source {
            kind,
            origin: Some(origin.into()),
            content: content.into(),
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
    /// assert_eq!(source.origin, None);
    /// assert_eq!(source.content, "function main(): void {}");
    /// ```
    pub fn inline<C: Into<String>>(kind: SourceKind, content: C) -> Source {
        Source {
            kind,
            origin: None,
            content: content.into(),
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
    /// let source = Source::new(SourceKind::Definition, "main.ara", "function main(): void {}");
    /// assert_eq!(source.name(), "main.ara");
    ///
    /// let source = Source::inline(SourceKind::Definition, "function main(): void {}");
    /// assert_eq!(source.name(), "<unknown>");
    /// ```
    pub fn name(&self) -> &str {
        match self.origin {
            Some(ref origin) => origin,
            None => DEFAULT_NAME,
        }
    }
}

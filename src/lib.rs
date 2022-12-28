use crate::error::Error;
use crate::source::Source;

pub mod error;
pub mod loader;
pub mod source;

#[derive(Debug)]
pub struct SourceMap {
    pub sources: Vec<Source>,
}

impl SourceMap {
    pub fn new(sources: Vec<Source>) -> SourceMap {
        SourceMap { sources }
    }

    pub fn add(&mut self, source: Source) {
        self.sources.push(source);
    }

    /// Get a source by its index.
    ///
    /// If the source is not found, `Error::SourceNotFound` is returned.
    pub fn get(&self, index: usize) -> Result<&Source, Error> {
        self.sources
            .get(index - 1)
            .ok_or_else(|| Error::SourceNotFound(index.to_string()))
    }

    /// Find a source by its origin.
    ///
    /// If the source is not found, `Error::SourceNotFound` is returned.
    pub fn named<T: Into<String>>(&self, name: T) -> Result<&Source, Error> {
        let name = name.into();

        self.sources
            .iter()
            .find(|source| source.origin == Some(name.clone()))
            .ok_or(Error::SourceNotFound(name))
    }

    /// Merge two source maps.
    ///
    /// The sources of the other source map are appended to the current source map.
    ///
    /// The other source map is emptied.
    pub fn merge(&mut self, other: &mut SourceMap) {
        self.sources.append(&mut other.sources);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::source::SourceKind;

    #[test]
    fn test_source_map() {
        let mut map = SourceMap::new(vec![]);

        map.add(Source::new(
            SourceKind::Script,
            "foo.ara",
            "function foo(): void {}",
        ));
        map.add(Source::new(
            SourceKind::Script,
            "bar.ara",
            "function bar(): void {}",
        ));

        assert_eq!(map.get(1).unwrap().origin, Some("foo.ara".to_string()));
        assert_eq!(map.get(2).unwrap().origin, Some("bar.ara".to_string()));
        assert!(map.get(3).is_err());

        assert_eq!(
            map.named("foo.ara").unwrap().origin,
            Some("foo.ara".to_string())
        );
        assert_eq!(
            map.named("bar.ara").unwrap().origin,
            Some("bar.ara".to_string())
        );
        assert!(map.named("baz.ara").is_err());

        let mut other = SourceMap::new(vec![]);

        other.add(Source::new(
            SourceKind::Script,
            "baz.ara",
            "function baz(): void {}",
        ));

        map.merge(&mut other);

        assert_eq!(map.get(1).unwrap().origin, Some("foo.ara".to_string()));
        assert_eq!(map.get(2).unwrap().origin, Some("bar.ara".to_string()));
        assert_eq!(map.get(3).unwrap().origin, Some("baz.ara".to_string()));

        assert!(other.get(1).is_err());
    }
}

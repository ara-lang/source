use crate::error::Error;
use crate::source::Source;
use crate::source::SourceKind;
use crate::SourceMap;

pub const ARA_SCRIPT_EXTENSION: &str = "ara";
pub const ARA_DEFINTION_EXTENSION: &str = "d.ara";

/// Load a source map from the given directories.
pub fn load_directories<T: Into<String>, C: Into<String>>(
    root: T,
    directories: Vec<C>,
) -> Result<SourceMap, Error> {
    let mut map = SourceMap::new(vec![]);

    let loader = DirectorySourceLoader::new(root.into());

    for directory in directories {
        loader.load_into(&directory.into(), &mut map)?;
    }

    Ok(map)
}

pub trait SourceLoader: std::fmt::Debug {
    /// Check if the given name is supported by this loader.
    ///
    /// If `true` is returned, `load` *MUST NOT* return `Error::InvalidSource`.
    fn supports(&self, name: &str) -> bool;

    /// Load a source map from the given name.
    ///
    /// The source name can contain a path, directory, or any other information.
    ///
    /// If the source name is not valid, `Error::InvalidSource` is returned.
    fn load(&self, name: &str) -> Result<SourceMap, Error>;

    /// Load a source from the given name and merge it into the given source map.
    fn load_into(&self, name: &str, map: &mut SourceMap) -> Result<(), Error> {
        let mut source = self.load(name)?;

        map.merge(&mut source);

        Ok(())
    }
}

#[derive(Debug)]
pub struct FileSourceLoader {
    pub root: String,
}

impl FileSourceLoader {
    pub fn new(root: String) -> FileSourceLoader {
        FileSourceLoader { root }
    }
}

impl SourceLoader for FileSourceLoader {
    fn supports(&self, name: &str) -> bool {
        if !name.starts_with(&self.root) {
            return false;
        }

        let path = std::path::Path::new(name);

        if !path.is_file() {
            return false;
        }

        match path.extension() {
            Some(extension) => {
                let extension = match extension.to_str() {
                    Some(extension) => extension,
                    None => {
                        return false;
                    }
                };

                if extension == ARA_SCRIPT_EXTENSION {
                    return true;
                }

                false
            }
            None => false,
        }
    }

    fn load(&self, name: &str) -> Result<SourceMap, Error> {
        if !self.supports(name) {
            return Err(Error::InvalidSource(format!(
                "source `{}` is not supported.",
                name
            )));
        }

        let kind = if name.ends_with(ARA_DEFINTION_EXTENSION) {
            SourceKind::Definition
        } else {
            SourceKind::Script
        };

        let path = std::path::Path::new(&name);

        std::fs::read_to_string(path)
            .map_err(Error::IoError)
            .map(|content| {
                SourceMap::new(vec![Source::new(
                    kind,
                    name.strip_prefix(&self.root).unwrap(),
                    content,
                )])
            })
    }
}

#[derive(Debug)]
pub struct DirectorySourceLoader {
    pub root: String,
    pub loaders: Vec<Box<dyn SourceLoader>>,
}

impl DirectorySourceLoader {
    pub fn new<T: Into<String>>(root: T) -> DirectorySourceLoader {
        let root = root.into();

        DirectorySourceLoader {
            root: root.clone(),
            loaders: vec![Box::new(FileSourceLoader::new(root))],
        }
    }

    pub fn add_loader(&mut self, loader: Box<dyn SourceLoader>) {
        self.loaders.push(loader);
    }
}

impl SourceLoader for DirectorySourceLoader {
    fn supports(&self, name: &str) -> bool {
        if !name.starts_with(&self.root) {
            return false;
        }

        let path = std::path::Path::new(name);

        if !path.is_dir() {
            return false;
        }

        let entries = match std::fs::read_dir(path) {
            Ok(entries) => entries,
            Err(_) => {
                return false;
            }
        };

        for entry in entries {
            let entry = match entry {
                Ok(entry) => entry,
                Err(_) => {
                    return false;
                }
            };

            let path = entry.path();
            let name = match path.to_str() {
                Some(name) => name,
                None => {
                    return false;
                }
            };

            if path.is_dir() && !self.supports(name) {
                return false;
            }
        }

        true
    }

    fn load(&self, name: &str) -> Result<SourceMap, Error> {
        if !self.supports(name) {
            return Err(Error::InvalidSource(format!(
                "source `{}` is not supported.",
                name
            )));
        }

        let path = std::path::Path::new(&name);

        let mut map = SourceMap::new(vec![]);

        let entries = std::fs::read_dir(path).unwrap();

        for entry in entries {
            let entry = entry.unwrap();
            let path = entry.path();
            let name = path.to_str().unwrap();

            if path.is_dir() {
                self.load_into(name, &mut map)?;
            } else {
                for loader in &self.loaders {
                    if loader.supports(name) {
                        loader.load_into(name, &mut map)?;

                        break;
                    }
                }
            }
        }

        Ok(map)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_directory() {
        let root = format!(
            "{}/examples/fixture/",
            std::env::var("CARGO_MANIFEST_DIR").unwrap()
        );

        let result = load_directories(
            root.clone(),
            vec![
                format!("{}src", root),
                format!("{}vendor/foo", root),
                format!("{}vendor/bar", root),
            ],
        );

        let map = result.unwrap();

        assert_eq!(map.sources.len(), 3);

        assert_eq!(map.named("src/main.ara").unwrap().kind, SourceKind::Script);
        assert_eq!(
            map.named("vendor/foo/write_line.d.ara").unwrap().kind,
            SourceKind::Definition
        );
        assert_eq!(
            map.named("vendor/bar/bar.d.ara").unwrap().kind,
            SourceKind::Definition
        );
    }
}

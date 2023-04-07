use std::path::Path;
use std::path::PathBuf;

use crate::error::Error;
use crate::source::Source;
use crate::source::SourceKind;
use crate::SourceMap;

pub const ARA_SCRIPT_EXTENSION: &str = "ara";
pub const ARA_DEFINTION_EXTENSION: &str = "d.ara";

/// Load a source map from the given directories.
pub fn load_directories<T: AsRef<Path>, C: AsRef<Path>>(
    root: T,
    directories: Vec<C>,
) -> Result<SourceMap, Error> {
    let mut map = SourceMap::new(vec![]);

    let loader = DirectorySourceLoader::new(&root);

    for directory in directories {
        loader.load_into(&directory, &mut map)?;
    }

    Ok(map)
}

pub trait SourceLoader: std::fmt::Debug {
    /// Check if the given name is supported by this loader.
    ///
    /// If `true` is returned, `load` *MUST NOT* return `Error::InvalidSource`.
    fn supports<T: AsRef<Path>>(&self, name: &T) -> bool;

    /// Load a source map from the given name.
    ///
    /// The source name can contain a path, directory, or any other information.
    ///
    /// If the source name is not valid, `Error::InvalidSource` is returned.
    fn load<T: AsRef<Path>>(&self, name: &T) -> Result<SourceMap, Error>;

    /// Load a source from the given name and merge it into the given source map.
    fn load_into<T: AsRef<Path>>(&self, name: &T, map: &mut SourceMap) -> Result<(), Error> {
        let mut source = self.load(name)?;

        map.merge(&mut source);

        Ok(())
    }
}

#[derive(Debug)]
pub struct FileSourceLoader {
    pub root: PathBuf,
}

impl FileSourceLoader {
    pub fn new<T: AsRef<Path>>(root: &T) -> FileSourceLoader {
        FileSourceLoader {
            root: root.as_ref().to_path_buf(),
        }
    }
}

impl SourceLoader for FileSourceLoader {
    fn supports<T: AsRef<Path>>(&self, file: &T) -> bool {
        let file = file.as_ref();
        let file = if file.is_relative() {
            self.root.join(file)
        } else {
            file.to_path_buf()
        };

        if !file.is_file() {
            return false;
        }

        match file.extension() {
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

    fn load<T: AsRef<Path>>(&self, file: &T) -> Result<SourceMap, Error> {
        let file = file.as_ref();

        if !self.supports(&file) {
            return Err(Error::InvalidSource(format!(
                "source `{}` is not supported.",
                file.to_string_lossy()
            )));
        }

        let file = if file.is_relative() {
            self.root.join(file)
        } else {
            file.to_path_buf()
        };

        let origin = file
            .strip_prefix(&self.root)
            .map(|path| path.to_string_lossy())
            .unwrap();

        let kind = if origin.ends_with(ARA_DEFINTION_EXTENSION) {
            SourceKind::Definition
        } else {
            SourceKind::Script
        };

        Ok(SourceMap::new(vec![Source::new(kind, &self.root, origin)]))
    }
}

#[derive(Debug)]
pub struct DirectorySourceLoader {
    pub root: PathBuf,

    loader: FileSourceLoader,
}

impl DirectorySourceLoader {
    pub fn new<T: AsRef<Path>>(root: &T) -> DirectorySourceLoader {
        DirectorySourceLoader {
            root: root.as_ref().to_path_buf(),
            loader: FileSourceLoader::new(root),
        }
    }
}

impl SourceLoader for DirectorySourceLoader {
    fn supports<T: AsRef<Path>>(&self, directory: &T) -> bool {
        let directory = directory.as_ref();
        let directory = if directory.is_relative() {
            self.root.join(directory)
        } else {
            directory.to_path_buf()
        };

        if !directory.starts_with(&self.root) {
            return false;
        }

        if !directory.is_dir() {
            return false;
        }

        true
    }

    fn load<T: AsRef<Path>>(&self, directory: &T) -> Result<SourceMap, Error> {
        let directory = directory.as_ref();
        if !self.supports(&directory) {
            return Err(Error::InvalidSource(format!(
                "source `{}` is not supported.",
                directory.to_string_lossy()
            )));
        }

        let directory = if directory.is_relative() {
            self.root.join(directory)
        } else {
            directory.to_path_buf()
        };

        let mut map = SourceMap::new(vec![]);

        let entries = std::fs::read_dir(directory)?;

        for entry in entries {
            let entry = entry.unwrap();
            let path = entry.path();

            if path.is_dir() {
                self.load_into(&path, &mut map)?;
            } else if self.loader.supports(&path) {
                self.loader.load_into(&path, &mut map)?;
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

        let result = load_directories(root, vec!["src", "vendor/foo", "vendor/bar"]);

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

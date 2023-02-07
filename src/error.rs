#[derive(Debug)]
pub enum Error {
    SourceNotFound(String),
    InvalidSource(String),
    IoError(std::io::Error),
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Error::IoError(error)
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::SourceNotFound(name) => write!(f, "source `{name}` not found."),
            Error::InvalidSource(message) => write!(f, "invalid source: {message}"),
            Error::IoError(error) => write!(f, "io error: {error}"),
        }
    }
}

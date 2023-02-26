use std::{
    io,
    path::{Path, PathBuf},
};

pub enum Error {
    /// Expected semantic-release configuration to exist at {path}
    ConfigurationFileNotFound { path: PathBuf },

    /// Unable to open file {path}
    FileOpenError { source: io::Error, path: PathBuf },

    /// Unable to read file {path}
    FileReadError { source: io::Error, path: PathBuf },

    /// Unable to parse semantic-release configuration file
    FileParseError {
        source: serde_json::Error,
        path: PathBuf,
    },

    /// Unable to serialize file
    FileSerializeError { source: serde_json::Error },

    /// Unable to write changes to file {path}
    FileWriteError { source: io::Error, path: PathBuf },
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::ConfigurationFileNotFound { path: _ } => None,
            Error::FileOpenError { source, path: _ } => Some(source),
            Error::FileReadError { source, path: _ } => Some(source),
            Error::FileParseError { source, path: _ } => Some(source),
            Error::FileSerializeError { source } => Some(source),
            Error::FileWriteError { source, path: _ } => Some(source),
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::ConfigurationFileNotFound { path } => {
                write!(f, "Expected configuration file does not exist {:?}", path)
            }
            Error::FileOpenError { source: _, path } => {
                write!(f, "Unable to open file {:?}", path)
            }
            Error::FileReadError { source: _, path } => {
                write!(f, "Unable to read file {:?}", path)
            }
            Error::FileParseError { source: _, path } => {
                write!(f, "Unable to parse file {:?}", path)
            }
            Error::FileSerializeError { source: _ } => {
                write!(f, "Unable to serialize semantic-release configuration")
            }
            Error::FileWriteError { source: _, path } => {
                write!(f, "{:?}", path)
            }
        }
    }
}

fn error_chain_fmt(
    e: &impl std::error::Error,
    f: &mut std::fmt::Formatter<'_>,
) -> std::fmt::Result {
    writeln!(f, "{}\n", e)?;
    let mut current = e.source();
    while let Some(cause) = current {
        writeln!(f, "Caused by:\n\t{}", cause)?;
        current = cause.source();
    }
    Ok(())
}

impl std::fmt::Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

impl Error {
    pub(crate) fn configuration_file_not_found_error(path: &Path) -> Error {
        Error::ConfigurationFileNotFound {
            path: path.to_owned(),
        }
    }

    pub(crate) fn file_open_error(source: io::Error, path: &Path) -> Error {
        Error::FileOpenError {
            source,
            path: path.to_owned(),
        }
    }

    pub(crate) fn file_read_error(source: io::Error, path: &Path) -> Error {
        Error::FileReadError {
            source,
            path: path.to_owned(),
        }
    }

    pub(crate) fn file_parse_error(source: serde_json::Error, path: &Path) -> Error {
        Error::FileParseError {
            source,
            path: path.to_owned(),
        }
    }

    pub(crate) fn file_serialize_error(source: serde_json::Error) -> Error {
        Error::FileSerializeError { source }
    }

    pub(crate) fn file_write_error(source: io::Error, path: &Path) -> Error {
        Error::FileWriteError {
            source,
            path: path.to_owned(),
        }
    }
}

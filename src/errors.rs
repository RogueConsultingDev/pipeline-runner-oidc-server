use std::io;

#[derive(Debug)]
pub(crate) enum Error {
    #[allow(dead_code)]
    ConfigError(config::ConfigError),
    #[allow(dead_code)]
    IoError(io::Error),
}

impl From<config::ConfigError> for Error {
    fn from(e: config::ConfigError) -> Self {
        Error::ConfigError(e)
    }
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Error::IoError(e)
    }
}

pub(crate) type Result<T> = std::result::Result<T, Error>;

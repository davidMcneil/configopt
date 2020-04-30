use std::{
    fmt,
    io::{self, Error as IoError, ErrorKind as IoErrorKind, Write},
    path::PathBuf,
    process,
};
use structopt::clap::{Error as ClapError, ErrorKind as ClapErrorKind};

#[derive(Debug)]
pub enum Error {
    ConfigGenerated(String),
    ConfigFile(PathBuf, IoError),
    ExpectedError(ClapError),
    Clap(ClapError),
}

macro_rules! wlnerr(
    ($($arg:tt)*) => ({
        use std::io::{Write, stderr};
        writeln!(&mut stderr(), $($arg)*).ok();
    })
);

impl Error {
    /// Should the message be written to `stdout` or not
    pub fn use_stderr(&self) -> bool {
        match self {
            Self::ConfigGenerated(_) => false,
            Self::ConfigFile(_, _) => true,
            Self::ExpectedError(e) => e.use_stderr(),
            Self::Clap(e) => e.use_stderr(),
        }
    }

    /// Prints the error to `stderr` or `stdout` and exits.
    ///
    /// This exits with a `0` when writing to `stdout` and `1` when writing to `stderr`.
    pub fn exit(&self) -> ! {
        self.exit_with_codes(1, 0)
    }

    /// Prints the error to `stderr` or `stdout` and exits with the specified code.
    pub fn exit_with_codes(&self, stdout_exit: i32, stderr_exit: i32) -> ! {
        if self.use_stderr() {
            wlnerr!("{}", self);
            process::exit(stderr_exit);
        }
        let out = io::stdout();
        writeln!(&mut out.lock(), "{}", self).expect("Error writing Error to stdout");
        process::exit(stdout_exit);
    }

    /// Is this error due to not being able to find a config file?
    pub fn config_file_not_found(&self) -> bool {
        match self {
            Self::ConfigFile(_, e) if e.kind() == IoErrorKind::NotFound => true,
            _ => false,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::ConfigGenerated(config) => write!(f, "{}", config),
            Self::ConfigFile(path, e) => write!(f, "Failed to parse config file '{}', err: {}", path.to_string_lossy(), e),
            Error::ExpectedError(e) => write!(f, "The `configopt` app generated an error, but the actual app did not. This should never happen. err: {}", e),
            Error::Clap(e) => write!(f, "{}", e),
        }
    }
}

impl std::error::Error for Error {}

impl From<ClapError> for Error {
    fn from(e: ClapError) -> Self {
        Self::Clap(e)
    }
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Self::Clap(ClapError::with_description(
            &e.to_string(),
            ClapErrorKind::Io,
        ))
    }
}

/// A `Result` that uses the `configopt` [`Error`](enum.Error.html) type
pub type Result<T> = std::result::Result<T, Error>;

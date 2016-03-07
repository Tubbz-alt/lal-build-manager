use std::error::Error;
use std::fmt;
use rustc_serialize::json;
use std::io;

#[derive(Debug)]
pub enum CliError {
    Io(io::Error),
    Parse(json::DecoderError),
    NoSuchComponent,
    MissingManifest,
    MissingConfig,
    MissingDependencies, // TODO: extend to take which dependency?
}

impl fmt::Display for CliError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CliError::Io(ref err) => err.fmt(f),
            CliError::Parse(ref err) => err.fmt(f),
            CliError::NoSuchComponent => write!(f, "No such component found"),
            CliError::MissingManifest => write!(f, "No manifest.json found"),
            CliError::MissingConfig => write!(f, "No ~/.lal/lalrc found"),
            CliError::MissingDependencies => write!(f, "Dependencies missing in INPUT"),
        }
    }
}

impl Error for CliError {
    fn description(&self) -> &str {
        match *self {
            CliError::Io(ref err) => err.description(),
            CliError::Parse(ref err) => err.description(),
            CliError::NoSuchComponent => "component not found",
            CliError::MissingManifest => "manifest not found",
            CliError::MissingConfig => "lalrc not found",
            CliError::MissingDependencies => "dependencies not all found",
        }
    }
}


impl From<io::Error> for CliError {
    fn from(err: io::Error) -> CliError {
        CliError::Io(err)
    }
}

impl From<json::DecoderError> for CliError {
    fn from(err: json::DecoderError) -> CliError {
        CliError::Parse(err)
    }
}
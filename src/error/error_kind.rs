use std::io;

#[derive(Copy, Clone, Debug)]
pub enum JoshutoErrorKind {
    // io related
    Io(io::ErrorKind),

    // environment variable not found
    EnvVarNotPresent,

    // parse error
    ParseError,
    ClipboardError,

    Glob,

    InvalidParameters,

    UnrecognizedArgument,
    UnrecognizedCommand,
}

impl std::convert::From<io::ErrorKind> for JoshutoErrorKind {
    fn from(err: io::ErrorKind) -> Self {
        Self::Io(err)
    }
}

impl std::convert::From<&globset::ErrorKind> for JoshutoErrorKind {
    fn from(_: &globset::ErrorKind) -> Self {
        Self::Glob
    }
}
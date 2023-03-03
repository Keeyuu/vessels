#[derive(Debug)]
pub struct Error {
    kind: String,
    msg: String,
}

enum ErrorKind {
    Io,
    Vessels,
}

impl ErrorKind {
    fn to_string(self) -> String {
        match self {
            ErrorKind::Io => "io".to_string(),
            ErrorKind::Vessels => "vessels".to_string(),
        }
    }
}

impl Error {
    pub fn new(msg: String) -> Self {
        Error {
            msg,
            kind: String::from(ErrorKind::Vessels.to_string()),
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "kind: {} msg: {}", self.kind, self.msg)
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(self)
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Self {
            kind: ErrorKind::Io.to_string(),
            msg: err.to_string(),
        }
    }
}

use std::fmt;
use std::process::Command;
use std::str::from_utf8;

fn main() {
    match execute() {
        Ok(ret) => {
            dbg!(from_utf8(&ret));
        }
        Err(err) => {
            panic!("{}", err);
        }
    };
}

pub enum CustomError {
    IoError(String),
    Utf8Error(String),
    ShellError(String),
}

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CustomError::IoError(message)
            | CustomError::Utf8Error(message)
            | CustomError::ShellError(message) => write!(f, "{}", message),
        }
    }
}

impl From<std::io::Error> for CustomError {
    fn from(e: std::io::Error) -> CustomError {
        CustomError::IoError(e.to_string())
    }
}

impl From<std::str::Utf8Error> for CustomError {
    fn from(e: std::str::Utf8Error) -> CustomError {
        CustomError::Utf8Error(e.to_string())
    }
}

fn execute() -> Result<Vec<u8>, CustomError> {
    let stdout = std::process::Stdio::piped();
    let result = Command::new("ls").stdout(stdout).args(&["-z"]).output()?;
    if result.status.success() {
        Ok(result.stdout)
    } else {
        Err(CustomError::ShellError(
            from_utf8(&result.stderr)?.to_string(),
        ))
    }
}

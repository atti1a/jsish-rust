use std::error;
use std::fmt;
use std::io;
use std::str;

#[derive(Debug)]
pub enum JsishError {
    Message(&'static str),
    IoError(io::Error),
}

pub type JsishResult<T> = Result<T, JsishError>;

impl error::Error for JsishError {
    fn description(&self) -> &str {
        match *self {
            JsishError::Message(msg) => msg,
            JsishError::IoError(ref err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            JsishError::IoError(ref err) => Some(err as &error::Error),
            _ => None,
        }
    }
}

impl fmt::Display for JsishError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            JsishError::Message(msg) => msg.fmt(f),
            JsishError::IoError(ref err) => err.fmt(f),
        }
    }
}

impl From<&'static str> for JsishError {
    fn from(err: &'static str) -> JsishError {
        JsishError::Message(err)
    }
}

impl From<io::Error> for JsishError {
    fn from(err: io::Error) -> JsishError {
        JsishError::IoError(err)
    }
}
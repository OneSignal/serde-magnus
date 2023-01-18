use magnus::{exception, ExceptionClass};
use std::{borrow::Cow, fmt};

#[derive(Debug)]
pub struct Error {
    inner: magnus::Error,
}

impl Error {
    pub fn new<Message: Into<Cow<'static, str>>>(class: ExceptionClass, message: Message) -> Error {
        Error {
            inner: magnus::Error::new(class, message),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        self.inner.fmt(formatter)
    }
}

impl std::error::Error for Error {}

impl serde::ser::Error for Error {
    fn custom<Message: std::fmt::Display>(message: Message) -> Self {
        Error::new(exception::runtime_error(), message.to_string())
    }
}

impl serde::de::Error for Error {
    fn custom<T: fmt::Display>(message: T) -> Self {
        Error::new(exception::runtime_error(), message.to_string())
    }
}

impl From<magnus::Error> for Error {
    fn from(error: magnus::Error) -> Error {
        Error { inner: error }
    }
}

impl From<Error> for magnus::Error {
    fn from(error: Error) -> magnus::Error {
        error.inner
    }
}

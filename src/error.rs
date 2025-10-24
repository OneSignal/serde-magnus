use magnus::{ExceptionClass, Ruby};
use std::{borrow::Cow, fmt};

#[derive(Debug)]
pub struct Error {
    inner: magnus::Error,
}

impl Error {
    pub fn new<Message>(class: ExceptionClass, message: Message) -> Error
    where
        Message: Into<Cow<'static, str>>,
    {
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
    fn custom<Message>(message: Message) -> Self
    where
        Message: fmt::Display,
    {
        Error::new(
            Ruby::get().unwrap().exception_runtime_error(),
            message.to_string(),
        )
    }
}

impl serde::de::Error for Error {
    fn custom<Message>(message: Message) -> Self
    where
        Message: fmt::Display,
    {
        Error::new(
            Ruby::get().unwrap().exception_runtime_error(),
            message.to_string(),
        )
    }

    fn invalid_type(unexpected: serde::de::Unexpected, expected: &dyn serde::de::Expected) -> Self {
        Error::new(
            Ruby::get().unwrap().exception_type_error(),
            format!("invalid type: expected {}, got {}", expected, unexpected),
        )
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

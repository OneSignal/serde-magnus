use magnus::{Error, RArray, Ruby, Value};
use std::convert::TryInto;
use tap::TapFallible;

/// For our purposes, stepping through an array by index is faster than `magnus::Enumerator`.
/// This is due to the fiber overhead of Ruby enumerators.
pub struct ArrayEnumerator<'r> {
    ruby: &'r Ruby,
    array: RArray,
    index: isize,
}

impl<'r> ArrayEnumerator<'r> {
    pub fn new(ruby: &'r Ruby, array: RArray) -> ArrayEnumerator<'r> {
        ArrayEnumerator {
            ruby,
            array,
            index: 0,
        }
    }

    fn current(&self) -> Result<Option<Value>, Error> {
        if let Ok(len) = self.array.len().try_into() {
            if self.index < len {
                self.array.entry(self.index).map(Some)
            } else {
                Ok(None)
            }
        } else {
            Err(Error::new(
                self.ruby.exception_range_error(),
                "array length out of range",
            ))
        }
    }
}

impl<'r> Iterator for ArrayEnumerator<'r> {
    type Item = Result<Value, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        self.current()
            .tap_ok(|item| {
                if item.is_some() {
                    self.index += 1
                }
            })
            .transpose()
    }
}

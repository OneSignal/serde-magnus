use magnus::{exception, Error, RArray, Value};
use std::convert::TryInto;
use tap::TapFallible;

/// For our purposes, stepping through an array by index is faster than `magnus::Enumerator`.
/// This is due to the fiber overhead of Ruby enumerators.
pub struct ArrayEnumerator {
    array: RArray,
    index: isize,
}

impl ArrayEnumerator {
    pub fn new(array: RArray) -> ArrayEnumerator {
        ArrayEnumerator { array, index: 0 }
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
                exception::range_error(),
                "array length out of range",
            ))
        }
    }
}

impl Iterator for ArrayEnumerator {
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

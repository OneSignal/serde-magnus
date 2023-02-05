use magnus::{exception, Error, RArray, Value};
use std::convert::TryInto;
use tap::TapFallible;

/// For a heterogeneous array, [`magnus::Enumerator::next`] returns an error on Ruby < 3.0:
///
/// ```text
/// #<FiberError: fiber called across stack rewinding barrier>
/// ```
///
/// This was fixed by [ruby/ruby#4606](https://github.com/ruby/ruby/pull/4606).
///
/// To appease older Rubies, step through an array by index rather than using a Ruby `Enumerator`.
///
/// TODO: remove this when dropping support for Ruby 2.7.
pub struct ArrayEnumerator {
    array: RArray,
    index: isize,
}

impl ArrayEnumerator {
    pub fn new(array: RArray) -> ArrayEnumerator {
        ArrayEnumerator { array, index: 0 }
    }

    fn peek(&self) -> Result<Option<Value>, Error> {
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
        self.peek()
            .tap_ok(|item| {
                if item.is_some() {
                    self.index += 1
                }
            })
            .transpose()
    }
}

use magnus::{Error, RString};
use serde_magnus::deserialize;

#[test]
fn test_deserializing_strings() -> Result<(), Error> {
    let _cleanup = unsafe { magnus::embed::init() };

    let output: char = deserialize(RString::new("☃"))?;
    assert_eq!('☃', output);

    let output: String = deserialize(RString::new("Hello, world!"))?;
    assert_eq!("Hello, world!", output);

    Ok(())
}

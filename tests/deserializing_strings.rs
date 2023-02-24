use magnus::{Error, RString};
use serde_bytes::{ByteBuf, Bytes};
use serde_magnus::deserialize;

#[test]
fn test_deserializing_strings() -> Result<(), Error> {
    let _cleanup = unsafe { magnus::embed::init() };

    let output: char = deserialize(RString::new("☃"))?;
    assert_eq!('☃', output);

    let output: Result<&str, Error> = deserialize(RString::new("Hello, world!"));
    assert_eq!(
        r#"TypeError: invalid type: expected a borrowed string, got string "Hello, world!""#,
        output.unwrap_err().to_string()
    );

    let output: String = deserialize(RString::new("Hello, world!"))?;
    assert_eq!("Hello, world!", output);

    let output: Result<&Bytes, Error> = deserialize(RString::new("Hello, world!"));
    assert_eq!(
        "TypeError: can't deserialize into byte slice",
        output.unwrap_err().to_string()
    );

    let output: ByteBuf = deserialize(RString::new("Hello, world!"))?;
    assert_eq!(b"Hello, world!", output.as_slice());

    Ok(())
}

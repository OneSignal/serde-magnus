use magnus::{encoding::EncodingCapable, Error, RString};
use serde_bytes::{ByteBuf, Bytes};
use serde_magnus::serialize;

#[test]
fn test_serializing_strings() -> Result<(), Error> {
    let ruby = unsafe { magnus::embed::init() };

    let output: RString = serialize(&ruby, &'☃')?;
    assert_eq!("☃", output.to_string()?);
    assert!(output.enc_get() == ruby.utf8_encindex());

    let output: RString = serialize(&ruby, &"Hello, world!")?;
    assert_eq!("Hello, world!", output.to_string()?);
    assert!(output.enc_get() == ruby.utf8_encindex());

    let output: RString = serialize(&ruby, &String::from("Hello, world!"))?;
    assert_eq!("Hello, world!", output.to_string()?);
    assert!(output.enc_get() == ruby.utf8_encindex());

    let output: RString = serialize(&ruby, &Bytes::new(b"Hello, world!"))?;
    assert_eq!(b"Hello, world!", unsafe { output.as_slice() });
    assert!(output.enc_get() == ruby.ascii8bit_encindex());

    let output: RString = serialize(&ruby, &ByteBuf::from(*b"Hello, world!"))?;
    assert_eq!(b"Hello, world!", unsafe { output.as_slice() });
    assert!(output.enc_get() == ruby.ascii8bit_encindex());

    Ok(())
}

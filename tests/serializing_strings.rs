use magnus::{encoding, encoding::EncodingCapable, Error, RString};
use serde_bytes::{ByteBuf, Bytes};
use serde_magnus::serialize;

#[test]
fn test_serializing_strings() -> Result<(), Error> {
    let _cleanup = unsafe { magnus::embed::init() };

    let output: RString = serialize(&'☃')?;
    assert_eq!("☃", output.to_string()?);
    assert!(output.enc_get() == encoding::Index::utf8());

    let output: RString = serialize(&"Hello, world!")?;
    assert_eq!("Hello, world!", output.to_string()?);
    assert!(output.enc_get() == encoding::Index::utf8());

    let output: RString = serialize(&String::from("Hello, world!"))?;
    assert_eq!("Hello, world!", output.to_string()?);
    assert!(output.enc_get() == encoding::Index::utf8());

    let output: RString = serialize(&Bytes::new(b"Hello, world!"))?;
    assert_eq!(b"Hello, world!", unsafe { output.as_slice() });
    assert!(output.enc_get() == encoding::Index::ascii8bit());

    let output: RString = serialize(&ByteBuf::from(*b"Hello, world!"))?;
    assert_eq!(b"Hello, world!", unsafe { output.as_slice() });
    assert!(output.enc_get() == encoding::Index::ascii8bit());

    Ok(())
}

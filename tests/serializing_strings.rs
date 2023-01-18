use magnus::{encoding, encoding::EncodingCapable, RString};
use serde_bytes::Bytes;
use serde_magnus::serialize;

#[test]
fn test_serializing_strings() {
    let _cleanup = unsafe { magnus::embed::init() };

    let output: RString = serialize(&'☃').unwrap();
    assert_eq!("☃", output.to_string().unwrap());
    assert!(output.enc_get() == encoding::Index::utf8());

    let output: RString = serialize("Hello, world!").unwrap();
    assert_eq!("Hello, world!", output.to_string().unwrap());
    assert!(output.enc_get() == encoding::Index::utf8());

    let output: RString = serialize(Bytes::new(b"Hello, world!")).unwrap();
    assert_eq!(b"Hello, world!", unsafe { output.as_slice() });
    assert!(output.enc_get() == encoding::Index::ascii8bit());
}

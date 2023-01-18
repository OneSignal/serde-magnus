use magnus::RString;
use serde_magnus::deserialize;

#[test]
fn test_deserializing_strings() {
    let _cleanup = unsafe { magnus::embed::init() };

    let output: char = deserialize(RString::new("☃")).unwrap();
    assert_eq!('☃', output);

    let output: String = deserialize(RString::new("Hello, world!")).unwrap();
    assert_eq!("Hello, world!", output);
}

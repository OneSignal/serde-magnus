use magnus::{RHash, RString, Symbol};
use serde::Deserialize;
use serde_magnus::deserialize;

#[derive(Deserialize, PartialEq, Debug)]
enum A {
    A,
    B(u64),
    C { message: String },
}

#[test]
fn test_deserializing_enums() {
    let _cleanup = unsafe { magnus::embed::init() };

    let input = RString::new("A");
    let output: A = deserialize(input).unwrap();
    assert_eq!(A::A, output);

    let input = RHash::new();
    input.aset("B", 123).unwrap();

    let output: A = deserialize(input).unwrap();
    assert_eq!(A::B(123), output);

    let value = RHash::new();
    value.aset(Symbol::new("message"), "Hello, world!").unwrap();

    let input = RHash::new();
    input.aset("C", value).unwrap();

    let output: A = deserialize(input).unwrap();
    assert_eq!(
        A::C {
            message: "Hello, world!".into()
        },
        output
    );
}

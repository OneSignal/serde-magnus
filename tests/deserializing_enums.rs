use magnus::{Error, RArray, RHash, RString, Symbol};
use serde::Deserialize;
use serde_magnus::deserialize;

#[derive(Deserialize, PartialEq, Debug)]
enum A {
    A,
    B(u64),
    C(u64, bool, String),
    D { message: String },
}

#[test]
fn test_deserializing_enums() -> Result<(), Error> {
    let _cleanup = unsafe { magnus::embed::init() };

    let input = RString::new("A");
    let output: A = deserialize(input)?;
    assert_eq!(A::A, output);

    let input = RHash::new();
    input.aset("B", 123)?;

    let output: A = deserialize(input)?;
    assert_eq!(A::B(123), output);

    let value = RArray::new();
    value.push(1234)?;
    value.push(true)?;
    value.push("Hello, world!")?;

    let input = RHash::new();
    input.aset("C", value)?;

    let output: A = deserialize(input).unwrap();
    assert_eq!(A::C(1234, true, "Hello, world!".into()), output);

    let value = RHash::new();
    value.aset(Symbol::new("message"), "Hello, world!")?;

    let input = RHash::new();
    input.aset("D", value)?;

    let output: A = deserialize(input)?;
    assert_eq!(
        A::D {
            message: "Hello, world!".into()
        },
        output
    );

    Ok(())
}

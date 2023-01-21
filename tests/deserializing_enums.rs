use magnus::{Error, RHash, RString, Symbol};
use serde::Deserialize;
use serde_magnus::deserialize;

#[derive(Deserialize, PartialEq, Debug)]
enum A {
    A,
    B(u64),
    C { message: String },
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

    let value = RHash::new();
    value.aset(Symbol::new("message"), "Hello, world!")?;

    let input = RHash::new();
    input.aset("C", value)?;

    let output: A = deserialize(input)?;
    assert_eq!(
        A::C {
            message: "Hello, world!".into()
        },
        output
    );

    Ok(())
}

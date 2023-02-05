use magnus::{eval, Error, RHash, RString};
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

    let input: RHash = eval!("{ 'B' => 123 }")?;
    let output: A = deserialize(input)?;
    assert_eq!(A::B(123), output);

    let input: RHash = eval!("{ 'C' => [1234, true, 'Hello, world!'] }")?;
    let output: A = deserialize(input).unwrap();
    assert_eq!(A::C(1234, true, "Hello, world!".into()), output);

    let input: RHash = eval!("{ 'D' => { message: 'Hello, world!' } }")?;
    let output: A = deserialize(input)?;
    assert_eq!(
        A::D {
            message: "Hello, world!".into()
        },
        output
    );

    Ok(())
}

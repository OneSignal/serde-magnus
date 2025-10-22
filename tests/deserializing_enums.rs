use magnus::{eval, Error, RHash};
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
    let ruby = unsafe { magnus::embed::init() };

    let input = ruby.str_new("A");
    let output: A = deserialize(&ruby, input)?;
    assert_eq!(A::A, output);

    let input: RHash = eval!(&ruby, "{ 'B' => 123 }")?;
    let output: A = deserialize(&ruby, input)?;
    assert_eq!(A::B(123), output);

    let input: RHash = eval!(&ruby, "{ 'C' => [1234, true, 'Hello, world!'] }")?;
    let output: A = deserialize(&ruby, input).unwrap();
    assert_eq!(A::C(1234, true, "Hello, world!".into()), output);

    let input: RHash = eval!(&ruby, "{ 'D' => { message: 'Hello, world!' } }")?;
    let output: A = deserialize(&ruby, input)?;
    assert_eq!(
        A::D {
            message: "Hello, world!".into()
        },
        output
    );

    Ok(())
}

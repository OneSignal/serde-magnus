use magnus::{eval, Error, RHash, RString};
use serde::Serialize;
use serde_magnus::serialize;

#[derive(Serialize)]
enum A {
    A,
    B(u64),
    C(u64, bool, String),
    D { message: String },
}

#[test]
fn test_serializing_enums() -> Result<(), Error> {
    let ruby = unsafe { magnus::embed::init() };

    let output: RString = serialize(&ruby, &A::A)?;
    assert!(eval!(&ruby, "output == 'A'", output)?);

    let input = A::B(123);
    let output: RHash = serialize(&ruby, &input)?;
    assert!(eval!(&ruby, "output == { 'B' => 123 }", output)?);

    let input = A::C(123, true, "Hello, world!".into());
    let output: RHash = serialize(&ruby, &input)?;
    assert!(eval!(
        &ruby,
        "output == { 'C' => [ 123, true, 'Hello, world!' ] }",
        output
    )?);

    let input = A::D {
        message: "Hello, world!".into(),
    };
    let output: RHash = serialize(&ruby, &input)?;
    assert!(eval!(
        &ruby,
        "output == { 'D' => { message: 'Hello, world!' } }",
        output
    )?);

    Ok(())
}

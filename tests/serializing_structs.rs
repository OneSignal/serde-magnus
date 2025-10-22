use magnus::{eval, value::ReprValue, Error, Integer, RArray, RHash, Value};
use serde::Serialize;
use serde_magnus::serialize;

#[derive(Serialize)]
struct A;

#[derive(Serialize)]
struct B(u64);

#[derive(Serialize)]
struct C(u64, bool, String);

#[derive(Serialize)]
struct D {
    message: String,
}

#[test]
fn test_serializing_structs() -> Result<(), Error> {
    let ruby = unsafe { magnus::embed::init() };

    let output: Value = serialize(&ruby, &A)?;
    assert!(output.is_nil());

    let input = B(123);
    let output: Integer = serialize(&ruby, &input)?;
    assert_eq!(123, output.to_u64()?);

    let input = C(1234, true, "Hello, world!".into());
    let output: RArray = serialize(&ruby, &input)?;
    assert!(eval!(
        &ruby,
        "output == [ 1234, true, 'Hello, world!' ]",
        output
    )?);

    let input = D {
        message: "Hello, world!".into(),
    };
    let output: RHash = serialize(&ruby, &input)?;
    assert!(eval!(
        &ruby,
        "output == { message: 'Hello, world!' }",
        output
    )?);

    Ok(())
}

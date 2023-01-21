use magnus::{Error, Integer, RHash, RString, Symbol, Value};
use serde::Serialize;
use serde_magnus::serialize;

#[derive(Serialize)]
struct A;

#[derive(Serialize)]
struct B(u64);

#[derive(Serialize)]
struct C {
    message: String,
}

#[test]
fn test_serializing_structs() -> Result<(), Error> {
    let _cleanup = unsafe { magnus::embed::init() };

    let output: Value = serialize(&A)?;
    assert!(output.is_nil());

    let output: Integer = serialize(&B(123))?;
    assert_eq!(123, output.to_u64()?);

    let output: RHash = serialize(&C {
        message: String::from("Hello, world!"),
    })?;
    assert_eq!(1, output.len());

    let message: RString = output.lookup(Symbol::new("message"))?;
    assert_eq!("Hello, world!", message.to_string()?);

    Ok(())
}

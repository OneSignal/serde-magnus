use magnus::{Error, Integer, RHash, RString, Symbol};
use serde::Serialize;
use serde_magnus::serialize;

#[derive(Serialize)]
enum A {
    A,
    B(u64),
    C { message: String },
}

#[test]
fn test_serializing_enums() -> Result<(), Error> {
    let _cleanup = unsafe { magnus::embed::init() };

    let output: RString = serialize(&A::A)?;
    assert_eq!("A", output.to_string()?);

    let output: RHash = serialize(&A::B(123))?;
    assert_eq!(1, output.len());

    let value: Integer = output.lookup("B")?;
    assert_eq!(123, value.to_u64()?);

    let output: RHash = serialize(&A::C {
        message: String::from("Hello, world!"),
    })?;
    assert_eq!(1, output.len());

    let value: RHash = output.lookup("C")?;
    assert_eq!(1, value.len());

    let message: RString = value.lookup(Symbol::new("message"))?;
    assert_eq!("Hello, world!", message.to_string()?);

    Ok(())
}

use magnus::{Integer, RHash, RString, Symbol, Value};
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
fn test_serializing_structs() {
    let _cleanup = unsafe { magnus::embed::init() };

    let output: Value = serialize(&A).unwrap();
    assert!(output.is_nil());

    let output: Integer = serialize(&B(123)).unwrap();
    assert_eq!(123, output.to_u64().unwrap());

    let output: RHash = serialize(&C {
        message: String::from("Hello, world!"),
    })
    .unwrap();
    assert_eq!(1, output.len());

    let message: RString = output.lookup(Symbol::new("message")).unwrap();
    assert_eq!("Hello, world!", message.to_string().unwrap());
}

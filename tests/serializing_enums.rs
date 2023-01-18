use magnus::{Integer, RHash, RString, Symbol};
use serde::Serialize;
use serde_magnus::serialize;

#[derive(Serialize)]
enum A {
    A,
    B(u64),
    C { message: String },
}

#[test]
fn test_serializing_enums() {
    let _cleanup = unsafe { magnus::embed::init() };

    let output: RString = serialize(&A::A).unwrap();
    assert_eq!("A", output.to_string().unwrap());

    let output: RHash = serialize(&A::B(123)).unwrap();
    assert_eq!(1, output.len());

    let value: Integer = output.lookup("B").unwrap();
    assert_eq!(123, value.to_u64().unwrap());

    let output: RHash = serialize(&A::C {
        message: String::from("Hello, world!"),
    })
    .unwrap();
    assert_eq!(1, output.len());

    let value: RHash = output.lookup("C").unwrap();
    assert_eq!(1, value.len());

    let message: RString = value.lookup(Symbol::new("message")).unwrap();
    assert_eq!("Hello, world!", message.to_string().unwrap());
}

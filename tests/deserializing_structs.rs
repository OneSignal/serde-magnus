use magnus::{Integer, RHash, Symbol, QNIL};
use serde::Deserialize;
use serde_magnus::deserialize;

#[derive(Deserialize, PartialEq, Debug)]
struct A;

#[derive(Deserialize, PartialEq, Debug)]
struct B(u64);

#[derive(Deserialize, PartialEq, Debug)]
struct C {
    message: String,
}

#[test]
fn test_deserializing_structs() {
    let _cleanup = unsafe { magnus::embed::init() };

    assert_eq!(A, deserialize(QNIL).unwrap());

    let input = Integer::from_u64(123);
    let output: B = deserialize(input).unwrap();
    assert_eq!(B(123), output);

    let input = RHash::new();
    input.aset(Symbol::new("message"), "Hello, world!").unwrap();

    let output: C = deserialize(input).unwrap();
    assert_eq!(
        C {
            message: "Hello, world!".into()
        },
        output
    );
}

use magnus::{eval, Error, Integer, RArray, RHash, QNIL};
use serde::Deserialize;
use serde_magnus::deserialize;

#[derive(Deserialize, PartialEq, Debug)]
struct A;

#[derive(Deserialize, PartialEq, Debug)]
struct B(u64);

#[derive(Deserialize, PartialEq, Debug)]
struct C(u64, bool, String);

#[derive(Deserialize, PartialEq, Debug)]
struct D {
    message: String,
}

#[test]
fn test_deserializing_structs() -> Result<(), Error> {
    let _cleanup = unsafe { magnus::embed::init() };

    assert_eq!(A, deserialize(QNIL)?);

    let input = Integer::from_u64(123);
    let output: B = deserialize(input)?;
    assert_eq!(B(123), output);

    let input: RArray = eval!("[ 123, true, 'Hello, world!' ]")?;
    let output: C = deserialize(input)?;
    assert_eq!(C(123, true, "Hello, world!".into()), output);

    let input: RHash = eval!("{ message: 'Hello, world!' }")?;
    let output: D = deserialize(input)?;
    assert_eq!(
        D {
            message: "Hello, world!".into()
        },
        output
    );

    Ok(())
}

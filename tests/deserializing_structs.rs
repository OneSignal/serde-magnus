use magnus::{eval, Error, IntoValue, RArray, RHash};
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
    let ruby = unsafe { magnus::embed::init() };

    assert_eq!(A, deserialize(&ruby, ruby.qnil().into_value_with(&ruby))?);

    let input = ruby.integer_from_u64(123);
    let output: B = deserialize(&ruby, input)?;
    assert_eq!(B(123), output);

    let input: RArray = eval!(&ruby, "[ 123, true, 'Hello, world!' ]")?;
    let output: C = deserialize(&ruby, input)?;
    assert_eq!(C(123, true, "Hello, world!".into()), output);

    let input: RHash = eval!(&ruby, "{ message: 'Hello, world!' }")?;
    let output: D = deserialize(&ruby, input)?;
    assert_eq!(
        D {
            message: "Hello, world!".into()
        },
        output
    );

    Ok(())
}

use magnus::Error;
use serde_magnus::deserialize;

#[test]
fn test_deserializing_integers() -> Result<(), Error> {
    let ruby = unsafe { magnus::embed::init() };

    let output: i64 = deserialize(&ruby, ruby.integer_from_i64(123))?;
    assert_eq!(123, output);

    let output: i64 = deserialize(&ruby, ruby.integer_from_i64(-123))?;
    assert_eq!(-123, output);

    let output: u64 = deserialize(&ruby, ruby.integer_from_u64(123))?;
    assert_eq!(123, output);

    Ok(())
}

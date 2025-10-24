use magnus::{Error, Integer};
use serde_magnus::serialize;

#[test]
fn test_serializing_integers() -> Result<(), Error> {
    let ruby = unsafe { magnus::embed::init() };

    let output: Integer = serialize(&ruby, &123i64)?;
    assert_eq!(123, output.to_i64()?);

    let output: Integer = serialize(&ruby, &-123i64)?;
    assert_eq!(-123, output.to_i64()?);

    let output: Integer = serialize(&ruby, &123u64)?;
    assert_eq!(123, output.to_u64()?);

    Ok(())
}

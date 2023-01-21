use magnus::{Error, Integer};
use serde_magnus::serialize;

#[test]
fn test_serializing_integers() -> Result<(), Error> {
    let _cleanup = unsafe { magnus::embed::init() };

    let output: Integer = serialize(&123i64)?;
    assert_eq!(123, output.to_i64()?);

    let output: Integer = serialize(&-123i64)?;
    assert_eq!(-123, output.to_i64()?);

    let output: Integer = serialize(&123u64)?;
    assert_eq!(123, output.to_u64()?);

    Ok(())
}

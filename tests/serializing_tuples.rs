use magnus::{Error, Integer, RArray, Value};
use serde_magnus::serialize;

#[test]
fn test_serializing_tuples() -> Result<(), Error> {
    let _cleanup = unsafe { magnus::embed::init() };

    let output: Value = serialize(&())?;
    assert!(output.is_nil());

    let output: RArray = serialize(&(1, 2, 3))?;
    assert_eq!(3, output.len());

    let entry: Integer = output.entry(0)?;
    assert_eq!(1, entry.to_u64()?);

    let entry: Integer = output.entry(1)?;
    assert_eq!(2, entry.to_u64()?);

    let entry: Integer = output.entry(2)?;
    assert_eq!(3, entry.to_u64()?);

    Ok(())
}

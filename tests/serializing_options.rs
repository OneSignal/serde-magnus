use magnus::{Error, Integer, Value};
use serde_magnus::serialize;

#[test]
fn test_serializing_options() -> Result<(), Error> {
    let _cleanup = unsafe { magnus::embed::init() };

    let input: Option<u64> = None;
    let output: Value = serialize(&input)?;
    assert!(output.is_nil());

    let input = Some(123);
    let output: Integer = serialize(&input)?;
    assert_eq!(123, output.to_u64()?);

    Ok(())
}

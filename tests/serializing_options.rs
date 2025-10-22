use magnus::{value::ReprValue, Error, Integer, Value};
use serde_magnus::serialize;

#[test]
fn test_serializing_options() -> Result<(), Error> {
    let ruby = unsafe { magnus::embed::init() };

    let input: Option<u64> = None;
    let output: Value = serialize(&ruby, &input)?;
    assert!(output.is_nil());

    let input = Some(123);
    let output: Integer = serialize(&ruby, &input)?;
    assert_eq!(123, output.to_u64()?);

    Ok(())
}

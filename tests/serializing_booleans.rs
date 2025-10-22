use magnus::{eval, Error, Value};
use serde_magnus::serialize;

#[test]
fn test_serializing_booleans() -> Result<(), Error> {
    let ruby = unsafe { magnus::embed::init() };

    let output: Value = serialize(&ruby, &true)?;
    assert!(eval!(&ruby, "output == true", output)?);

    let output: Value = serialize(&ruby, &false)?;
    assert!(eval!(&ruby, "output == false", output)?);

    Ok(())
}

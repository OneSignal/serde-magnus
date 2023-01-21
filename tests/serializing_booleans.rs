use magnus::{
    value::{Qfalse, Qtrue},
    Error, Value,
};
use serde_magnus::serialize;

#[test]
fn test_serializing_booleans() -> Result<(), Error> {
    let _cleanup = unsafe { magnus::embed::init() };

    let output: Value = serialize(&true)?;
    assert!(Qtrue::from_value(output).is_some());

    let output: Value = serialize(&false)?;
    assert!(Qfalse::from_value(output).is_some());

    Ok(())
}

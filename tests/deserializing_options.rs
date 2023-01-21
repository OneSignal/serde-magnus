use magnus::{Error, Integer, QNIL};
use serde_magnus::deserialize;

#[test]
fn test_deserializing_options() -> Result<(), Error> {
    let _cleanup = unsafe { magnus::embed::init() };

    let output: Option<u64> = deserialize(QNIL)?;
    assert_eq!(None, output);

    let output: Option<u64> = deserialize(Integer::from_u64(123))?;
    assert_eq!(Some(123), output);

    Ok(())
}

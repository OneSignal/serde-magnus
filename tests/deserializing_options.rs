use magnus::{Error, IntoValue};
use serde_magnus::deserialize;

#[test]
fn test_deserializing_options() -> Result<(), Error> {
    let ruby = unsafe { magnus::embed::init() };

    let output: Option<u64> = deserialize(&ruby, ruby.qnil().into_value_with(&ruby))?;
    assert_eq!(None, output);

    let output: Option<u64> = deserialize(&ruby, ruby.integer_from_u64(123))?;
    assert_eq!(Some(123), output);

    Ok(())
}

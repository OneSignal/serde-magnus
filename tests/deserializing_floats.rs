use magnus::Error;
use serde_magnus::deserialize;

#[test]
fn test_deserializing_floats() -> Result<(), Error> {
    let ruby = unsafe { magnus::embed::init() };

    let output: f64 = deserialize(&ruby, ruby.float_from_f64(1.23))?;
    assert_eq!(1.23, output);

    Ok(())
}

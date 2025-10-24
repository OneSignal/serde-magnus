use magnus::{value::ReprValue, Error};
use serde_magnus::deserialize;

#[test]
fn test_deserializing_booleans() -> Result<(), Error> {
    let ruby = unsafe { magnus::embed::init() };

    let output: bool = deserialize(&ruby, ruby.qtrue().as_value())?;
    assert_eq!(true, output);

    let output: bool = deserialize(&ruby, ruby.qfalse().as_value())?;
    assert_eq!(false, output);

    Ok(())
}

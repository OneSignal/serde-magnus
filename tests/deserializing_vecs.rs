use magnus::{eval, Error, RArray};
use serde_magnus::deserialize;

#[test]
fn test_deserializing_vecs() -> Result<(), Error> {
    let ruby = unsafe { magnus::embed::init() };

    let input: RArray = eval!(&ruby, "[ 1, 2, 3 ]")?;
    let output: Vec<i64> = deserialize(&ruby, input)?;
    assert_eq!(&[1, 2, 3], &output[..]);

    Ok(())
}

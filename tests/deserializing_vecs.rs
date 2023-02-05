use magnus::{eval, Error, RArray};
use serde_magnus::deserialize;

#[test]
fn test_deserializing_vecs() -> Result<(), Error> {
    let _cleanup = unsafe { magnus::embed::init() };

    let input: RArray = eval!("[ 1, 2, 3 ]")?;
    let output: Vec<i64> = deserialize(input)?;
    assert_eq!(&[1, 2, 3], &output[..]);

    Ok(())
}

use magnus::{eval, Error, RArray};
use serde_magnus::serialize;

#[test]
fn test_serializing_arrays() -> Result<(), Error> {
    let ruby = unsafe { magnus::embed::init() };

    let input: [u64; 3] = [1, 2, 3];
    let output: RArray = serialize(&ruby, &input)?;
    assert!(eval!(&ruby, "output == [ 1, 2, 3 ]", output)?);

    Ok(())
}

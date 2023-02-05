use magnus::{eval, Error, RArray};
use serde_magnus::serialize;

#[test]
fn test_serializing_arrays() -> Result<(), Error> {
    let _cleanup = unsafe { magnus::embed::init() };

    let input: [u64; 3] = [1, 2, 3];
    let output: RArray = serialize(&input)?;
    assert!(eval!("output == [ 1, 2, 3 ]", output)?);

    Ok(())
}

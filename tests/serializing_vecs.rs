use magnus::{Error, Integer, RArray};
use serde_magnus::serialize;

#[test]
fn test_serializing_vecs() -> Result<(), Error> {
    let _cleanup = unsafe { magnus::embed::init() };

    let input: Vec<u64> = Vec::from([1, 2, 3]);
    let output: RArray = serialize(&input)?;
    assert_eq!(3, output.len());
    assert_eq!(1, output.entry::<Integer>(0)?.to_u64()?);
    assert_eq!(2, output.entry::<Integer>(1)?.to_u64()?);
    assert_eq!(3, output.entry::<Integer>(2)?.to_u64()?);

    Ok(())
}

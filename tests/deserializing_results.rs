use magnus::{eval, Error, RHash};
use serde_magnus::deserialize;

#[test]
fn test_deserializing_results() -> Result<(), Error> {
    let _cleanup = unsafe { magnus::embed::init() };

    let input: RHash = eval!("{ 'Ok' => 1234 }")?;
    let output: Result<u64, String> = deserialize(input)?;
    assert_eq!(Ok(1234), output);

    let input: RHash = eval!("{ 'Err' => 'something went wrong' }")?;
    let output: Result<u64, String> = deserialize(input)?;
    assert_eq!(Err("something went wrong".into()), output);

    Ok(())
}

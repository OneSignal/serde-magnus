use magnus::{eval, Error, RHash};
use serde_magnus::deserialize;

#[test]
fn test_deserializing_results() -> Result<(), Error> {
    let ruby = unsafe { magnus::embed::init() };

    let input: RHash = eval!(&ruby, "{ 'Ok' => 1234 }")?;
    let output: Result<u64, String> = deserialize(&ruby, input)?;
    assert_eq!(Ok(1234), output);

    let input: RHash = eval!(&ruby, "{ 'Err' => 'something went wrong' }")?;
    let output: Result<u64, String> = deserialize(&ruby, input)?;
    assert_eq!(Err("something went wrong".into()), output);

    Ok(())
}

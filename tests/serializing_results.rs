use magnus::{eval, Error, RHash};
use serde_magnus::serialize;

#[test]
fn test_serializing_results() -> Result<(), Error> {
    let ruby = unsafe { magnus::embed::init() };

    let input: Result<u64, &str> = Ok(1234);
    let output: RHash = serialize(&ruby, &input)?;
    assert!(eval!(&ruby, "output == { 'Ok' => 1234 }", output)?);

    let input: Result<u64, &str> = Err("something went wrong");
    let output: RHash = serialize(&ruby, &input)?;
    assert!(eval!(
        &ruby,
        "output == { 'Err' => 'something went wrong' }",
        output
    )?);

    Ok(())
}

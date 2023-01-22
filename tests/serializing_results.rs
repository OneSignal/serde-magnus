use magnus::{Error, Integer, RHash, RString};
use serde_magnus::serialize;

#[test]
fn test_serializing_results() -> Result<(), Error> {
    let _cleanup = unsafe { magnus::embed::init() };

    let input: Result<u64, &str> = Ok(1234);
    let output: RHash = serialize(&input)?;
    assert_eq!(1, output.len());
    let value: Integer = output.lookup("Ok")?;
    assert_eq!(1234, value.to_u64()?);

    let input: Result<u64, &str> = Err("something went wrong");
    let output: RHash = serialize(&input)?;
    assert_eq!(1, output.len());
    let value: RString = output.lookup("Err")?;
    assert_eq!("something went wrong", value.to_string()?);

    Ok(())
}

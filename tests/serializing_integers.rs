use magnus::Integer;
use serde_magnus::serialize;

#[test]
fn test_serializing_integers() {
    let _cleanup = unsafe { magnus::embed::init() };

    let output: Integer = serialize(&123i64).unwrap();
    assert_eq!(123, output.to_i64().unwrap());

    let output: Integer = serialize(&-123i64).unwrap();
    assert_eq!(-123, output.to_i64().unwrap());

    let output: Integer = serialize(&123u64).unwrap();
    assert_eq!(123, output.to_u64().unwrap());
}

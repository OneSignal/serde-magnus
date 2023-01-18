use magnus::Integer;
use serde_magnus::deserialize;

#[test]
fn test_deserializing_integers() {
    let _cleanup = unsafe { magnus::embed::init() };

    let output: i64 = deserialize(Integer::from_i64(123)).unwrap();
    assert_eq!(123, output);

    let output: i64 = deserialize(Integer::from_i64(-123)).unwrap();
    assert_eq!(-123, output);

    let output: u64 = deserialize(Integer::from_u64(123)).unwrap();
    assert_eq!(123, output);
}

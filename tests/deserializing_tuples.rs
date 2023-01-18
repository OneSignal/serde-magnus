use magnus::{Integer, RArray, QNIL};
use serde_magnus::deserialize;

#[test]
fn test_deserializing_tuples() {
    let _cleanup = unsafe { magnus::embed::init() };

    assert_eq!((), deserialize(QNIL).unwrap());

    let input: RArray = RArray::new();
    input.push(Integer::from_i64(1)).unwrap();
    input.push(Integer::from_i64(2)).unwrap();
    input.push(Integer::from_i64(3)).unwrap();

    let output: (i64, i64, i64) = deserialize(input).unwrap();
    assert_eq!((1, 2, 3), output);
}

use magnus::{Integer, QNIL};
use serde_magnus::deserialize;

#[test]
fn test_deserializing_options() {
    let _cleanup = unsafe { magnus::embed::init() };

    let output: Option<u64> = deserialize(QNIL).unwrap();
    assert_eq!(None, output);

    let output: Option<u64> = deserialize(Integer::from_u64(123)).unwrap();
    assert_eq!(Some(123), output);
}

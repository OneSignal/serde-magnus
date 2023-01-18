use magnus::{QFALSE, QTRUE};
use serde_magnus::deserialize;

#[test]
fn test_deserializing_booleans() {
    let _cleanup = unsafe { magnus::embed::init() };

    let output: bool = deserialize(QTRUE).unwrap();
    assert_eq!(true, output);

    let output: bool = deserialize(QFALSE).unwrap();
    assert_eq!(false, output);
}

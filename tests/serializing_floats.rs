use magnus::Float;
use serde_magnus::serialize;

#[test]
fn test_serializing_floats() {
    let _cleanup = unsafe { magnus::embed::init() };

    let float: Float = serialize(&1.23).unwrap();
    assert_eq!(1.23, float.to_f64());
}

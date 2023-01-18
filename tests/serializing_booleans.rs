use magnus::{
    value::{Qfalse, Qtrue},
    Value,
};
use serde_magnus::serialize;

#[test]
fn test_serializing_booleans() {
    let _cleanup = unsafe { magnus::embed::init() };

    let output: Value = serialize(&true).unwrap();
    assert!(Qtrue::from_value(output).is_some());

    let output: Value = serialize(&false).unwrap();
    assert!(Qfalse::from_value(output).is_some());
}

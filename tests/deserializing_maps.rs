use magnus::RHash;
use serde_magnus::deserialize;
use std::collections::HashMap;

#[test]
fn test_deserializing_maps() {
    let _cleanup = unsafe { magnus::embed::init() };

    let input: RHash = RHash::new();
    input.aset("Yes", "No").unwrap();
    input.aset("Stop", "Go").unwrap();
    input.aset("High", "Low").unwrap();

    let output: HashMap<String, String> = deserialize(input).unwrap();
    assert_eq!(3, output.len());
    assert_eq!(Some(&"No".into()), output.get("Yes"));
    assert_eq!(Some(&"Go".into()), output.get("Stop"));
    assert_eq!(Some(&"Low".into()), output.get("High"));
}

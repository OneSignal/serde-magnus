use magnus::{RHash, RString};
use serde_magnus::serialize;
use std::collections::HashMap;

#[test]
fn test_serializing_maps() {
    let _cleanup = unsafe { magnus::embed::init() };

    let mut input: HashMap<&str, &str> = HashMap::new();
    input.insert("Yes", "No");
    input.insert("Stop", "Go");
    input.insert("High", "Low");

    let output: RHash = serialize(&input).unwrap();
    assert_eq!(3, output.len());

    let value: RString = output.lookup("Yes").unwrap();
    assert_eq!("No", value.to_string().unwrap());

    let value: RString = output.lookup("Stop").unwrap();
    assert_eq!("Go", value.to_string().unwrap());

    let value: RString = output.lookup("High").unwrap();
    assert_eq!("Low", value.to_string().unwrap());
}

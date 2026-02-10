use chrono::{DateTime, Utc};
use magnus::{eval, Error, Value};
use serde_magnus::deserialize;

#[test]
fn test_deserializing_times() -> Result<(), Error> {
    let ruby = unsafe { magnus::embed::init() };
    // Time#iso8601 doesn't exist unless time is required
    ruby.require("time")?;

    let time: Value = eval!(&ruby, "Time.now")?;
    let string: String = deserialize(&ruby, time)?;
    let _output: DateTime<Utc> = string.parse().unwrap();

    let time: Value = eval!(&ruby, "Time.parse('2026-01-27T10:40:33.545967000-05:00')")?;
    let string: String = deserialize(&ruby, time)?;
    let output: DateTime<Utc> = string.parse().unwrap();
    // The hour changes from 10 to 15 because the input timestamp isn't UTC
    assert_eq!(output.to_string(), "2026-01-27 15:40:33.545967 UTC");

    Ok(())
}

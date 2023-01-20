use crate::error::Error;
use magnus::{RHash, Value};
use std::ops::Deref;

pub fn nest<Data>(variant: &'static str, data: Data) -> Result<Value, Error>
where
    Data: Deref<Target = Value>,
{
    let hash = RHash::new();
    hash.aset(variant, *data)?;
    Ok(*hash)
}

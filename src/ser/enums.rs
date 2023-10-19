use crate::error::Error;
use magnus::{IntoValue, RHash, Value};

pub fn nest<Data>(variant: &'static str, data: Data) -> Result<Value, Error>
where
    Data: IntoValue,
{
    let hash = RHash::new();
    hash.aset(variant, data)?;
    Ok(hash.into_value())
}

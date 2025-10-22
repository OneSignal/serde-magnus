use super::Serializer;
use crate::error::Error;
use magnus::{value::ReprValue, IntoValue, RHash, Ruby, Value};
use serde::{ser::SerializeMap, Serialize};

pub struct MapSerializer<'r> {
    ruby: &'r Ruby,
    hash: RHash,
    key: Value,
}

impl<'r> MapSerializer<'r> {
    pub fn new(ruby: &'r Ruby, hash: RHash) -> MapSerializer<'r> {
        MapSerializer {
            ruby,
            hash,
            key: ruby.qnil().as_value(),
        }
    }
}

impl<'r> SerializeMap for MapSerializer<'r> {
    type Ok = Value;
    type Error = Error;

    fn serialize_key<Key>(&mut self, key: &Key) -> Result<(), Self::Error>
    where
        Key: Serialize + ?Sized,
    {
        self.key = key.serialize(Serializer::new(self.ruby))?;
        Ok(())
    }

    fn serialize_value<Value>(&mut self, value: &Value) -> Result<(), Self::Error>
    where
        Value: Serialize + ?Sized,
    {
        self.hash
            .aset(self.key, value.serialize(Serializer::new(self.ruby))?)
            .map_err(Into::into)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(self.hash.into_value_with(self.ruby))
    }
}

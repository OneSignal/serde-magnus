use super::Serializer;
use crate::error::Error;
use magnus::{
    value::{qnil, ReprValue},
    IntoValue, RHash, Value,
};
use serde::{ser::SerializeMap, Serialize};

pub struct MapSerializer {
    hash: RHash,
    key: Value,
}

impl MapSerializer {
    pub fn new(hash: RHash) -> MapSerializer {
        MapSerializer {
            hash,
            key: qnil().as_value(),
        }
    }
}

impl SerializeMap for MapSerializer {
    type Ok = Value;
    type Error = Error;

    fn serialize_key<Key>(&mut self, key: &Key) -> Result<(), Self::Error>
    where
        Key: Serialize + ?Sized,
    {
        self.key = key.serialize(Serializer)?;
        Ok(())
    }

    fn serialize_value<Value>(&mut self, value: &Value) -> Result<(), Self::Error>
    where
        Value: Serialize + ?Sized,
    {
        self.hash
            .aset(self.key, value.serialize(Serializer)?)
            .map_err(Into::into)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(self.hash.into_value())
    }
}

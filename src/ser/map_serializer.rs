use super::Serializer;
use crate::error::Error;
use magnus::{RHash, Value};
use serde::{ser::SerializeMap, Serialize};

pub struct MapSerializer {
    hash: RHash,
    key: Value,
}

impl MapSerializer {
    pub fn new(hash: RHash) -> MapSerializer {
        MapSerializer {
            hash,
            key: Value::default(),
        }
    }
}

impl SerializeMap for MapSerializer {
    type Ok = Value;
    type Error = Error;

    fn serialize_key<Key: Serialize + ?Sized>(&mut self, key: &Key) -> Result<(), Self::Error> {
        self.key = key.serialize(Serializer)?;
        Ok(())
    }

    fn serialize_value<Value: Serialize + ?Sized>(
        &mut self,
        value: &Value,
    ) -> Result<(), Self::Error> {
        self.hash
            .aset(self.key, value.serialize(Serializer)?)
            .map_err(Into::into)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(self.hash.into())
    }
}

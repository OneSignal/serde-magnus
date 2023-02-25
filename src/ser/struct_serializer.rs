use super::Serializer;
use crate::error::Error;
use magnus::{RHash, Symbol, Value};
use serde::{ser::SerializeStruct, Serialize};

pub struct StructSerializer {
    hash: RHash,
}

impl StructSerializer {
    pub fn new(hash: RHash) -> StructSerializer {
        StructSerializer { hash }
    }
}

impl SerializeStruct for StructSerializer {
    type Ok = Value;
    type Error = Error;

    fn serialize_field<Value>(
        &mut self,
        name: &'static str,
        value: &Value,
    ) -> Result<(), Self::Error>
    where
        Value: Serialize + ?Sized,
    {
        self.hash
            .aset(Symbol::new(name), value.serialize(Serializer)?)
            .map_err(Into::into)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(*self.hash)
    }
}

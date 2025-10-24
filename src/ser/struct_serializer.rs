use super::Serializer;
use crate::error::Error;
use magnus::{IntoValue, RHash, Ruby, Value};
use serde::{ser::SerializeStruct, Serialize};

pub struct StructSerializer<'r> {
    ruby: &'r Ruby,
    hash: RHash,
}

impl<'r> StructSerializer<'r> {
    pub fn new(ruby: &'r Ruby, hash: RHash) -> StructSerializer<'r> {
        StructSerializer { ruby, hash }
    }
}

impl<'r> SerializeStruct for StructSerializer<'r> {
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
            .aset(
                self.ruby.to_symbol(name),
                value.serialize(Serializer::new(self.ruby))?,
            )
            .map_err(Into::into)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(self.hash.into_value_with(self.ruby))
    }
}

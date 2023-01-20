use super::{enums::nest, Serializer};
use crate::error::Error;
use magnus::{RHash, Symbol, Value};
use serde::{ser::SerializeStructVariant, Serialize};

pub struct StructVariantSerializer {
    variant: &'static str,
    hash: RHash,
}

impl StructVariantSerializer {
    pub fn new(variant: &'static str, hash: RHash) -> StructVariantSerializer {
        StructVariantSerializer { variant, hash }
    }
}

impl SerializeStructVariant for StructVariantSerializer {
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
        nest(self.variant, self.hash)
    }
}

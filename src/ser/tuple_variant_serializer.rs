use super::Serializer;
use crate::error::Error;
use magnus::{RArray, RHash, Value};
use serde::{ser::SerializeTupleVariant, Serialize};

pub struct TupleVariantSerializer {
    variant: &'static str,
    array: RArray,
}

impl TupleVariantSerializer {
    pub fn new(variant: &'static str, array: RArray) -> TupleVariantSerializer {
        TupleVariantSerializer { variant, array }
    }
}

impl SerializeTupleVariant for TupleVariantSerializer {
    type Ok = Value;
    type Error = Error;

    fn serialize_field<Field: Serialize + ?Sized>(
        &mut self,
        field: &Field,
    ) -> Result<(), Self::Error> {
        self.array
            .push(field.serialize(Serializer)?)
            .map_err(Into::into)
    }

    fn end(self) -> Result<Self::Ok, self::Error> {
        let hash = RHash::new();
        hash.aset(self.variant, self.array)?;
        Ok(hash.into())
    }
}

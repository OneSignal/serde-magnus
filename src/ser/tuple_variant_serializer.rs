use super::{enums::nest, Serializer};
use crate::error::Error;
use magnus::{RArray, Value};
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

    fn serialize_field<Field>(&mut self, field: &Field) -> Result<(), Self::Error>
    where
        Field: Serialize + ?Sized,
    {
        self.array
            .push(field.serialize(Serializer)?)
            .map_err(Into::into)
    }

    fn end(self) -> Result<Self::Ok, self::Error> {
        nest(self.variant, self.array)
    }
}

use super::Serializer;
use crate::error::Error;
use magnus::{RArray, Value};
use serde::{
    ser::{SerializeSeq, SerializeTuple, SerializeTupleStruct},
    Serialize,
};

pub struct SeqSerializer {
    array: RArray,
}

impl SeqSerializer {
    pub fn new(array: RArray) -> SeqSerializer {
        SeqSerializer { array }
    }
}

impl SerializeSeq for SeqSerializer {
    type Ok = Value;
    type Error = Error;

    fn serialize_element<Element: Serialize + ?Sized>(
        &mut self,
        element: &Element,
    ) -> Result<(), Self::Error> {
        self.array
            .push(element.serialize(Serializer)?)
            .map_err(Into::into)
    }

    fn end(self) -> Result<Self::Ok, self::Error> {
        Ok(self.array.into())
    }
}

impl SerializeTuple for SeqSerializer {
    type Ok = Value;
    type Error = Error;

    fn serialize_element<Element: Serialize + ?Sized>(
        &mut self,
        element: &Element,
    ) -> Result<(), Self::Error> {
        <Self as SerializeSeq>::serialize_element(self, element)
    }

    fn end(self) -> Result<Self::Ok, self::Error> {
        <Self as SerializeSeq>::end(self)
    }
}

impl SerializeTupleStruct for SeqSerializer {
    type Ok = Value;
    type Error = Error;

    fn serialize_field<Field: Serialize + ?Sized>(
        &mut self,
        field: &Field,
    ) -> Result<(), Self::Error> {
        <Self as SerializeSeq>::serialize_element(self, field)
    }

    fn end(self) -> Result<Self::Ok, self::Error> {
        <Self as SerializeSeq>::end(self)
    }
}

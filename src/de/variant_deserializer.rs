use super::{ArrayDeserializer, Deserializer, HashDeserializer};
use crate::error::Error;
use magnus::{RArray, RHash, Value};
use serde::de::{DeserializeSeed, Unexpected, VariantAccess};

pub struct VariantDeserializer {
    value: Value,
}

impl VariantDeserializer {
    pub fn new(value: Value) -> VariantDeserializer {
        VariantDeserializer { value }
    }
}

impl<'i> VariantAccess<'i> for VariantDeserializer {
    type Error = Error;

    fn unit_variant(self) -> Result<(), Self::Error> {
        if self.value.is_nil() {
            Ok(())
        } else {
            Err(serde::de::Error::invalid_type(
                #[allow(clippy::unnecessary_to_owned)]
                Unexpected::Other(&unsafe { self.value.classname() }.into_owned()),
                &"unit variant",
            ))
        }
    }

    fn newtype_variant_seed<Seed>(self, seed: Seed) -> Result<Seed::Value, Self::Error>
    where
        Seed: DeserializeSeed<'i>,
    {
        seed.deserialize(Deserializer::new(self.value))
    }

    fn tuple_variant<Visitor>(
        self,
        _len: usize,
        visitor: Visitor,
    ) -> Result<Visitor::Value, Self::Error>
    where
        Visitor: serde::de::Visitor<'i>,
    {
        if let Some(array) = RArray::from_value(self.value) {
            visitor.visit_seq(&mut ArrayDeserializer::new(array))
        } else {
            Err(serde::de::Error::invalid_type(
                #[allow(clippy::unnecessary_to_owned)]
                Unexpected::Other(&unsafe { self.value.classname() }.into_owned()),
                &"tuple variant",
            ))
        }
    }

    fn struct_variant<Visitor>(
        self,
        _fields: &'static [&'static str],
        visitor: Visitor,
    ) -> Result<Visitor::Value, Self::Error>
    where
        Visitor: serde::de::Visitor<'i>,
    {
        if let Some(hash) = RHash::from_value(self.value) {
            visitor.visit_map(&mut HashDeserializer::new(hash)?)
        } else {
            Err(serde::de::Error::invalid_type(
                #[allow(clippy::unnecessary_to_owned)]
                Unexpected::Other(&unsafe { self.value.classname() }.into_owned()),
                &"struct variant",
            ))
        }
    }
}

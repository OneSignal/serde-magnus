use super::VariantDeserializer;
use crate::error::Error;
use magnus::Value;
use serde::de::{DeserializeSeed, EnumAccess, IntoDeserializer};

pub struct EnumDeserializer {
    variant: String,
    value: Value,
}

impl EnumDeserializer {
    pub fn new(variant: String, value: Value) -> EnumDeserializer {
        EnumDeserializer { variant, value }
    }
}

impl<'i> EnumAccess<'i> for EnumDeserializer {
    type Variant = VariantDeserializer;
    type Error = Error;

    fn variant_seed<Seed>(self, seed: Seed) -> Result<(Seed::Value, Self::Variant), Error>
    where
        Seed: DeserializeSeed<'i>,
    {
        let deserializer = VariantDeserializer::new(self.value);

        seed.deserialize(self.variant.into_deserializer())
            .map(|value| (value, deserializer))
    }
}

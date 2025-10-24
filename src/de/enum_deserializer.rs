use super::VariantDeserializer;
use crate::error::Error;
use magnus::{Ruby, Value};
use serde::de::{DeserializeSeed, EnumAccess, IntoDeserializer};

pub struct EnumDeserializer<'r> {
    ruby: &'r Ruby,
    variant: String,
    value: Value,
}

impl<'r> EnumDeserializer<'r> {
    pub fn new(ruby: &'r Ruby, variant: String, value: Value) -> EnumDeserializer<'r> {
        EnumDeserializer {
            ruby,
            variant,
            value,
        }
    }
}

impl<'r, 'i> EnumAccess<'i> for EnumDeserializer<'r> {
    type Variant = VariantDeserializer<'r>;
    type Error = Error;

    fn variant_seed<Seed>(self, seed: Seed) -> Result<(Seed::Value, Self::Variant), Error>
    where
        Seed: DeserializeSeed<'i>,
    {
        let deserializer = VariantDeserializer::new(self.ruby, self.value);

        seed.deserialize(self.variant.into_deserializer())
            .map(|value| (value, deserializer))
    }
}

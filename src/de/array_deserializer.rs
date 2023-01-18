use super::Deserializer;
use crate::error::Error;
use magnus::{Enumerator, RArray};
use serde::de::{DeserializeSeed, SeqAccess};

pub struct ArrayDeserializer {
    entries: Enumerator,
}

impl ArrayDeserializer {
    pub fn new(array: RArray) -> ArrayDeserializer {
        ArrayDeserializer {
            entries: array.each(),
        }
    }
}

impl<'i> SeqAccess<'i> for ArrayDeserializer {
    type Error = Error;

    fn next_element_seed<Seed>(&mut self, seed: Seed) -> Result<Option<Seed::Value>, Self::Error>
    where
        Seed: DeserializeSeed<'i>,
    {
        match self.entries.next() {
            Some(Ok(entry)) => seed.deserialize(Deserializer::new(entry)).map(Some),
            Some(Err(error)) => Err(Error::from(error)),
            None => Ok(None),
        }
    }
}

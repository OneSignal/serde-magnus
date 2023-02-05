use super::{arrays::ArrayEnumerator, Deserializer};
use crate::error::Error;
use magnus::RArray;
use serde::de::{DeserializeSeed, SeqAccess};

pub struct ArrayDeserializer {
    entries: ArrayEnumerator,
}

impl ArrayDeserializer {
    pub fn new(array: RArray) -> ArrayDeserializer {
        ArrayDeserializer {
            entries: ArrayEnumerator::new(array),
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
            Some(Err(error)) => Err(error.into()),
            None => Ok(None),
        }
    }
}

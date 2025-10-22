use super::{array_enumerator::ArrayEnumerator, Deserializer};
use crate::error::Error;
use magnus::{RArray, Ruby};
use serde::de::{DeserializeSeed, SeqAccess};

pub struct ArrayDeserializer<'r> {
    ruby: &'r Ruby,
    entries: ArrayEnumerator<'r>,
}

impl<'r> ArrayDeserializer<'r> {
    pub fn new(ruby: &'r Ruby, array: RArray) -> ArrayDeserializer<'r> {
        ArrayDeserializer {
            ruby,
            entries: ArrayEnumerator::new(ruby, array),
        }
    }
}

impl<'r, 'i> SeqAccess<'i> for ArrayDeserializer<'r> {
    type Error = Error;

    fn next_element_seed<Seed>(&mut self, seed: Seed) -> Result<Option<Seed::Value>, Self::Error>
    where
        Seed: DeserializeSeed<'i>,
    {
        match self.entries.next() {
            Some(Ok(entry)) => seed
                .deserialize(Deserializer::new(self.ruby, entry))
                .map(Some),
            Some(Err(error)) => Err(error.into()),
            None => Ok(None),
        }
    }
}

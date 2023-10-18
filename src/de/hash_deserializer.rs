use super::{array_enumerator::ArrayEnumerator, Deserializer};
use crate::error::Error;
use magnus::{exception, value::ReprValue, RHash};
use serde::de::{DeserializeSeed, MapAccess};
use std::iter::Peekable;

pub struct HashDeserializer {
    hash: RHash,
    keys: Peekable<ArrayEnumerator>,
}

impl HashDeserializer {
    pub fn new(hash: RHash) -> Result<HashDeserializer, Error> {
        Ok(HashDeserializer {
            hash,
            keys: ArrayEnumerator::new(hash.funcall("keys", ())?).peekable(),
        })
    }
}

impl<'i> MapAccess<'i> for HashDeserializer {
    type Error = Error;

    fn next_key_seed<Seed>(&mut self, seed: Seed) -> Result<Option<Seed::Value>, Self::Error>
    where
        Seed: DeserializeSeed<'i>,
    {
        match self.keys.peek() {
            Some(&Ok(key)) => seed.deserialize(Deserializer::new(key)).map(Some),

            Some(Err(error)) => Err(Error::new(
                exception::runtime_error(),
                format!("encountered unexpected error: {}", error),
            )),

            None => Ok(None),
        }
    }

    fn next_value_seed<Seed>(&mut self, seed: Seed) -> Result<Seed::Value, Self::Error>
    where
        Seed: DeserializeSeed<'i>,
    {
        match self.keys.next() {
            Some(Ok(key)) => seed.deserialize(Deserializer::new(self.hash.aref(key)?)),
            Some(Err(error)) => Err(error.into()),
            None => Err(Error::new(exception::index_error(), "index out of range")),
        }
    }
}

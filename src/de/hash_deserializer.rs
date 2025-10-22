use super::{array_enumerator::ArrayEnumerator, Deserializer};
use crate::error::Error;
use magnus::{value::ReprValue, RHash, Ruby};
use serde::de::{DeserializeSeed, MapAccess};
use std::iter::Peekable;

pub struct HashDeserializer<'r> {
    ruby: &'r Ruby,
    hash: RHash,
    keys: Peekable<ArrayEnumerator<'r>>,
}

impl<'r> HashDeserializer<'r> {
    pub fn new(ruby: &'r Ruby, hash: RHash) -> Result<HashDeserializer<'r>, Error> {
        Ok(HashDeserializer {
            ruby,
            hash,
            keys: ArrayEnumerator::new(ruby, hash.funcall("keys", ())?).peekable(),
        })
    }
}

impl<'r, 'i> MapAccess<'i> for HashDeserializer<'r> {
    type Error = Error;

    fn next_key_seed<Seed>(&mut self, seed: Seed) -> Result<Option<Seed::Value>, Self::Error>
    where
        Seed: DeserializeSeed<'i>,
    {
        match self.keys.peek() {
            Some(&Ok(key)) => seed
                .deserialize(Deserializer::new(self.ruby, key))
                .map(Some),

            Some(Err(error)) => Err(Error::new(
                self.ruby.exception_runtime_error(),
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
            Some(Ok(key)) => seed.deserialize(Deserializer::new(self.ruby, self.hash.aref(key)?)),
            Some(Err(error)) => Err(error.into()),
            None => Err(Error::new(
                self.ruby.exception_index_error(),
                "index out of range",
            )),
        }
    }
}

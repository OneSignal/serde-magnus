use magnus::{
    value::{Qfalse, Qtrue, ReprValue},
    Fixnum, Float, RArray, RBignum, RHash, RString, Ruby, Symbol, Value,
};

use serde::forward_to_deserialize_any;

use super::{ArrayDeserializer, EnumDeserializer, HashDeserializer};
use crate::error::Error;

pub struct Deserializer<'r> {
    ruby: &'r Ruby,
    value: Value,
}

impl<'r> Deserializer<'r> {
    pub fn new(ruby: &'r Ruby, value: Value) -> Deserializer<'r> {
        Deserializer { ruby, value }
    }
}

impl<'r, 'i> serde::Deserializer<'i> for Deserializer<'r> {
    type Error = Error;

    fn deserialize_any<Visitor>(self, visitor: Visitor) -> Result<Visitor::Value, Self::Error>
    where
        Visitor: serde::de::Visitor<'i>,
    {
        if self.value.is_nil() {
            return visitor.visit_unit();
        }

        if let Some(qtrue) = Qtrue::from_value(self.value) {
            return visitor.visit_bool(qtrue.to_bool());
        }

        if let Some(qfalse) = Qfalse::from_value(self.value) {
            return visitor.visit_bool(qfalse.to_bool());
        }

        if let Some(fixnum) = Fixnum::from_value(self.value) {
            return visitor.visit_i64(fixnum.to_i64());
        }

        if let Some(bignum) = RBignum::from_value(self.value) {
            return visitor.visit_i64(bignum.to_i64()?);
        }

        if let Some(float) = Float::from_value(self.value) {
            return visitor.visit_f64(float.to_f64());
        }

        if let Some(string) = RString::from_value(self.value) {
            return visitor.visit_str(string.to_string()?.as_str());
        }

        if let Some(symbol) = Symbol::from_value(self.value) {
            return visitor.visit_str(symbol.name()?.to_string().as_str());
        }

        if let Some(array) = RArray::from_value(self.value) {
            return visitor.visit_seq(ArrayDeserializer::new(self.ruby, array));
        }

        if let Some(hash) = RHash::from_value(self.value) {
            return visitor.visit_map(HashDeserializer::new(self.ruby, hash)?);
        }

        Err(Error::new(
            self.ruby.exception_type_error(),
            format!(
                "can't deserialize {}",
                unsafe { self.value.classname() }.into_owned()
            ),
        ))
    }

    fn deserialize_bytes<Visitor>(self, _visitor: Visitor) -> Result<Visitor::Value, Self::Error>
    where
        Visitor: serde::de::Visitor<'i>,
    {
        Err(Error::new(
            self.ruby.exception_type_error(),
            "can't deserialize into byte slice",
        ))
    }

    fn deserialize_byte_buf<Visitor>(self, visitor: Visitor) -> Result<Visitor::Value, Self::Error>
    where
        Visitor: serde::de::Visitor<'i>,
    {
        if let Some(string) = RString::from_value(self.value) {
            visitor.visit_byte_buf(unsafe { string.as_slice() }.to_owned())
        } else {
            Err(Error::new(
                self.ruby.exception_type_error(),
                format!(
                    "no implicit conversion of {} to String",
                    unsafe { self.value.classname() }.into_owned()
                ),
            ))
        }
    }

    fn deserialize_option<Visitor>(self, visitor: Visitor) -> Result<Visitor::Value, Self::Error>
    where
        Visitor: serde::de::Visitor<'i>,
    {
        if self.value.is_nil() {
            visitor.visit_none()
        } else {
            visitor.visit_some(self)
        }
    }

    fn deserialize_enum<Visitor>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        visitor: Visitor,
    ) -> Result<Visitor::Value, Self::Error>
    where
        Visitor: serde::de::Visitor<'i>,
    {
        if let Some(variant) = RString::from_value(self.value) {
            return visitor.visit_enum(EnumDeserializer::new(
                self.ruby,
                variant.to_string()?,
                self.ruby.qnil().as_value(),
            ));
        }

        if let Some(hash) = RHash::from_value(self.value) {
            if hash.len() == 1 {
                let keys: RArray = hash.funcall("keys", ())?;
                let key: String = keys.entry(0)?;
                let value = hash
                    .get(key.as_str())
                    .unwrap_or_else(|| self.ruby.qnil().as_value());

                return visitor.visit_enum(EnumDeserializer::new(self.ruby, key, value));
            } else {
                return Err(Error::new(
                    self.ruby.exception_type_error(),
                    format!("can't deserialize Hash of length {} to Enum", hash.len()),
                ));
            }
        }

        Err(Error::new(
            self.ruby.exception_type_error(),
            format!(
                "can't deserialize {} to Enum",
                unsafe { self.value.classname() }.into_owned()
            ),
        ))
    }

    fn deserialize_newtype_struct<Visitor>(
        self,
        _name: &'static str,
        visitor: Visitor,
    ) -> Result<Visitor::Value, Self::Error>
    where
        Visitor: serde::de::Visitor<'i>,
    {
        visitor.visit_newtype_struct(self)
    }

    fn deserialize_ignored_any<Visitor>(
        self,
        visitor: Visitor,
    ) -> Result<Visitor::Value, Self::Error>
    where
        Visitor: serde::de::Visitor<'i>,
    {
        visitor.visit_unit()
    }

    forward_to_deserialize_any! {
        <Visitor: Visitor<'i>>
        bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string
        unit unit_struct seq tuple tuple_struct map struct identifier
    }
}

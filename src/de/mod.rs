mod deserializer;

mod array_deserializer;
mod enum_deserializer;
mod hash_deserializer;
mod variant_deserializer;

use self::deserializer::Deserializer;

use self::{
    array_deserializer::ArrayDeserializer, enum_deserializer::EnumDeserializer,
    hash_deserializer::HashDeserializer, variant_deserializer::VariantDeserializer,
};

use magnus::{Error, Value};
use serde::Deserialize;
use std::ops::Deref;

/// Deserialize a Ruby [`Value`][`magnus::Value`] to Rust.
///
/// See [`crate::serialize`] for the expected Ruby formats of various Rust types.
///
/// ```
/// use magnus::{eval, RHash};
/// use serde::Deserialize;
/// use serde_magnus::deserialize;
/// # let _cleanup = unsafe { magnus::embed::init() };
///
/// #[derive(Deserialize, Debug)]
/// struct A {
///     b: B,
///     c: [i32; 3],
///     d: Option<D>,
/// }
///
/// #[derive(Deserialize, PartialEq, Debug)]
/// struct B(u32);
///
/// #[derive(Deserialize, PartialEq, Debug)]
/// enum D {
///     A { a: String },
/// }
///
/// let input: RHash = eval(
///     r#"
///     {
///       b: 1234,
///       c: [123, -456, 789],
///       d: {
///         "A" => { a: "test" }
///       }
///     }
///     "#,
/// )
/// .unwrap();
///
/// let output: A = deserialize(input).unwrap();
/// assert_eq!(B(1234), output.b);
/// assert_eq!([123, -456, 789], output.c);
/// assert_eq!(
///     Some(D::A {
///         a: String::from("test")
///     }),
///     output.d
/// );
/// ```
pub fn deserialize<'i, Input, Output>(input: Input) -> Result<Output, Error>
where
    Input: Deref<Target = Value>,
    Output: Deserialize<'i>,
{
    Output::deserialize(Deserializer::new(*input)).map_err(Into::into)
}

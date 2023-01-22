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
/// )?;
///
/// let output: A = deserialize(input)?;
/// assert_eq!(B(1234), output.b);
/// assert_eq!([123, -456, 789], output.c);
/// assert_eq!(
///     Some(D::A {
///         a: "test".into()
///     }),
///     output.d
/// );
///
/// # Ok::<(), magnus::Error>(())
/// ```
///
/// ### Borrowing
///
/// Although [`serde::Deserialize`] is implemented for `&str`, `&[u8]`, and `&std::path::Path`, it
/// is not possible to deserialize to these types from Ruby values. Any attempt to do so will
/// result in an `Err`. The reason is that it cannot be guaranteed the underlying Ruby data will
/// outlive the borrow.
///
/// Use owned equivalents such as `String`, `Vec<u8>`, and [`std::path::PathBuf`] instead.
pub fn deserialize<'i, Input, Output>(input: Input) -> Result<Output, Error>
where
    Input: Deref<Target = Value>,
    Output: Deserialize<'i>,
{
    Output::deserialize(Deserializer::new(*input)).map_err(Into::into)
}

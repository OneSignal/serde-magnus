mod deserializer;

mod array_deserializer;
mod arrays;
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
/// ### Primitive types
///
/// #### Unit type
///
/// ```
/// # let _cleanup = unsafe { magnus::embed::init() };
/// #
/// use magnus::{eval, Value};
/// use serde_magnus::deserialize;
///
/// let input: Value = eval!("nil")?;
/// let output: () = deserialize(&input)?;
/// assert_eq!((), output);
/// #
/// # Ok::<(), magnus::Error>(())
/// ```
///
/// #### Booleans
///
/// ```
/// # use magnus::{eval, Value};
/// # use serde_magnus::deserialize;
/// #
/// # let _cleanup = unsafe { magnus::embed::init() };
/// #
/// let input: Value = eval!("true")?;
/// let output: bool = deserialize(&input)?;
/// assert_eq!(true, output);
/// #
/// # Ok::<(), magnus::Error>(())
/// ```
///
/// #### Integers
///
/// ```
/// # use magnus::{eval, Value};
/// # use serde_magnus::deserialize;
/// #
/// # let _cleanup = unsafe { magnus::embed::init() };
/// #
/// let input: Value = eval!("1234")?;
/// let output: i64 = deserialize(&input)?;
/// assert_eq!(1234, output);
/// #
/// # Ok::<(), magnus::Error>(())
/// ```
///
/// #### Floats
///
/// ```
/// # use magnus::{eval, Value};
/// # use serde_magnus::deserialize;
/// #
/// # let _cleanup = unsafe { magnus::embed::init() };
/// #
/// let input: Value = eval!("3.14")?;
/// let output: f64 = deserialize(&input)?;
/// assert_eq!(3.14, output);
/// #
/// # Ok::<(), magnus::Error>(())
/// ```
///
/// #### Strings
///
/// ```
/// # use magnus::{eval, Value};
/// # use serde_magnus::deserialize;
/// #
/// # let _cleanup = unsafe { magnus::embed::init() };
/// #
/// let input: Value = eval!(r#""Hello, world!""#)?;
/// let output: String = deserialize(&input)?;
/// assert_eq!("Hello, world!", output);
/// #
/// # Ok::<(), magnus::Error>(())
/// ```
///
/// ### `Option`
///
/// ```
/// # use magnus::{eval, Value};
/// # use serde_magnus::deserialize;
/// #
/// # let _cleanup = unsafe { magnus::embed::init() };
/// #
/// let input: Value = eval!("nil")?;
/// let output: Option<i64> = deserialize(&input)?;
/// assert_eq!(None, output);
///
/// let input: Value = eval!("1234")?;
/// let output: Option<i64> = deserialize(&input)?;
/// assert_eq!(Some(1234), output);
/// #
/// # Ok::<(), magnus::Error>(())
/// ```
///
/// ### `Result`
///
/// ```
/// # use magnus::{eval, Value};
/// # use serde_magnus::deserialize;
/// #
/// # let _cleanup = unsafe { magnus::embed::init() };
/// #
/// let input: Value = eval!("{ 'Ok' => 1234 }")?;
/// let output: Result<i64, String> = deserialize(&input)?;
/// assert_eq!(Ok(1234), output);
///
/// let input: Value = eval!("{ 'Err' => 'something went wrong' }")?;
/// let output: Result<i64, String> = deserialize(&input)?;
/// assert_eq!(Err("something went wrong".into()), output);
/// #
/// # Ok::<(), magnus::Error>(())
/// ```
///
/// ### Structs
///
/// #### Unit structs
///
/// ```
/// # use magnus::{eval, Value};
/// # use serde_magnus::deserialize;
/// #
/// # let _cleanup = unsafe { magnus::embed::init() };
/// #
/// use serde::Deserialize;
///
/// #[derive(PartialEq, Debug, Deserialize)]
/// struct Foo;
///
/// let input: Value = eval!("nil")?;
/// let output: Foo = deserialize(&input)?;
/// assert_eq!(Foo, output);
/// #
/// # Ok::<(), magnus::Error>(())
/// ```
///
/// #### Newtype structs
///
/// ```
/// # use magnus::{eval, Value};
/// # use serde_magnus::deserialize;
/// #
/// # let _cleanup = unsafe { magnus::embed::init() };
/// #
/// use serde::Deserialize;
///
/// #[derive(PartialEq, Debug, Deserialize)]
/// struct Foo(u16);
///
/// let input: Value = eval!("1234")?;
/// let output: Foo = deserialize(&input)?;
/// assert_eq!(Foo(1234), output);
/// #
/// # Ok::<(), magnus::Error>(())
/// ```
///
/// #### Tuple structs
///
/// ```
/// # use magnus::{eval, Value};
/// # use serde_magnus::deserialize;
/// #
/// # let _cleanup = unsafe { magnus::embed::init() };
/// #
/// use serde::Deserialize;
///
/// #[derive(PartialEq, Debug, Deserialize)]
/// struct Foo(u16, bool, String);
///
/// let input: Value = eval!("[123, true, 'Hello, world!']")?;
/// let output: Foo = deserialize(&input)?;
/// assert_eq!(Foo(123, true, "Hello, world!".into()), output);
/// #
/// # Ok::<(), magnus::Error>(())
/// ```
///
/// #### Structs with named fields
///
/// ```
/// # use magnus::{eval, Value};
/// # use serde::Deserialize;
/// # use serde_magnus::deserialize;
/// #
/// # let _cleanup = unsafe { magnus::embed::init() };
/// #
/// #[derive(PartialEq, Debug, Deserialize)]
/// struct Foo {
///     bar: u16,
///     baz: bool,
///     glorp: String
/// }
///
/// let input: Value = eval!("{ bar: 1234, baz: true, glorp: 'Hello, world!' }")?;
/// let output: Foo = deserialize(&input)?;
/// assert_eq!(
///     Foo { bar: 1234, baz: true, glorp: "Hello, world!".into() },
///     output
/// );
/// #
/// # Ok::<(), magnus::Error>(())
/// ```
///
/// ### Enums
///
/// ```
/// # use serde::Deserialize;
/// #[derive(PartialEq, Debug, Deserialize)]
/// enum Foo {
///     Bar,
///     Baz(u16),
///     Glorp(u16, bool, String),
///     Quux {
///         frob: u16,
///         wally: bool,
///         plugh: String
///     }
/// }
/// ```
///
/// #### Unit enum variants
///
/// ```
/// # use magnus::{eval, Value};
/// # use serde::Deserialize;
/// # use serde_magnus::deserialize;
/// #
/// # let _cleanup = unsafe { magnus::embed::init() };
/// #
/// # #[derive(PartialEq, Debug, Deserialize)]
/// # enum Foo { Bar }
/// #
/// let input: Value = eval!("'Bar'")?;
/// let output: Foo = deserialize(&input)?;
/// assert_eq!(Foo::Bar, output);
/// #
/// # Ok::<(), magnus::Error>(())
/// ```
///
/// #### Newtype enum variants
///
/// ```
/// # use magnus::{eval, Value};
/// # use serde::Deserialize;
/// # use serde_magnus::deserialize;
/// #
/// # let _cleanup = unsafe { magnus::embed::init() };
/// #
/// # #[derive(PartialEq, Debug, Deserialize)]
/// # enum Foo { Baz(u16) }
/// #
/// let input: Value = eval!("{ 'Baz' => 1234 }")?;
/// let output: Foo = deserialize(&input)?;
/// assert_eq!(Foo::Baz(1234), output);
/// #
/// # Ok::<(), magnus::Error>(())
/// ```
///
/// #### Tuple enum variants
///
/// ```
/// # use magnus::{eval, Value};
/// # use serde::Deserialize;
/// # use serde_magnus::deserialize;
/// #
/// # let _cleanup = unsafe { magnus::embed::init() };
/// #
/// # #[derive(PartialEq, Debug, Deserialize)]
/// # enum Foo { Glorp(u16, bool, String) }
/// #
/// let input: Value = eval!("{ 'Glorp' => [123, true, 'Hello, world!'] }")?;
/// let output: Foo = deserialize(&input)?;
/// assert_eq!(Foo::Glorp(123, true, "Hello, world!".into()), output);
/// #
/// # Ok::<(), magnus::Error>(())
/// ```
///
/// #### Struct enum variants
///
/// ```
/// # use magnus::{eval, Value};
/// # use serde::Deserialize;
/// # use serde_magnus::deserialize;
/// #
/// # let _cleanup = unsafe { magnus::embed::init() };
/// #
/// # #[derive(PartialEq, Debug, Deserialize)]
/// # enum Foo {
/// #     Quux {
/// #         frob: u16,
/// #         wally: bool,
/// #         plugh: String
/// #     }
/// # }
/// #
/// let input: Value = eval!(r#"
///     {
///       "Quux" => {
///         frob: 1234,
///         wally: true,
///         plugh: "Hello, world!"
///       }
///     }
/// "#)?;
/// let output: Foo = deserialize(&input)?;
/// assert_eq!(
///     Foo::Quux { frob: 1234, wally: true, plugh: "Hello, world!".into() },
///     output
/// );
/// #
/// # Ok::<(), magnus::Error>(())
/// ```
///
/// ### Compound types
///
/// #### Tuples
///
/// ```
/// # use magnus::{eval, Value};
/// # use serde_magnus::deserialize;
/// #
/// # let _cleanup = unsafe { magnus::embed::init() };
/// #
/// let input: Value = eval!("[123, true, 'Hello, world!']")?;
/// let output: (i16, bool, String) = deserialize(&input)?;
/// assert_eq!((123, true, "Hello, world!".into()), output);
/// #
/// # Ok::<(), magnus::Error>(())
/// ```
///
/// #### Arrays
///
/// ```
/// # use magnus::{eval, Value};
/// # use serde_magnus::deserialize;
/// #
/// # let _cleanup = unsafe { magnus::embed::init() };
/// #
/// let input: Value = eval!("[123, 456, 789]")?;
/// let output: [i64; 3] = deserialize(&input)?;
/// assert_eq!([123, 456, 789], output);
/// #
/// # Ok::<(), magnus::Error>(())
/// ```
///
/// ### Collections
///
/// #### Sequences
///
/// ```
/// # use magnus::{eval, Value};
/// # use serde_magnus::deserialize;
/// #
/// # let _cleanup = unsafe { magnus::embed::init() };
/// #
/// let input: Value = eval!("[123, 456, 789]")?;
/// let output: Vec<u64> = deserialize(&input)?;
/// assert_eq!(vec![123, 456, 789], output);
/// #
/// # Ok::<(), magnus::Error>(())
/// ```
///
/// #### Maps
///
/// ```
/// # use magnus::{eval, Value};
/// # use serde_magnus::deserialize;
/// #
/// # let _cleanup = unsafe { magnus::embed::init() };
/// #
/// use std::collections::HashMap;
///
/// let input: Value = eval!(r#"
///     {
///       "yes" => "no",
///       "stop" => "go",
///       "high" => "low",
///       "goodbye" => "hello"
///     }
/// "#)?;
/// let output: HashMap<String, String> = deserialize(&input)?;
/// assert_eq!(4, output.len());
/// assert_eq!(Some(&String::from("no")), output.get("yes"));
/// #
/// # Ok::<(), magnus::Error>(())
/// ```
///
/// ### Borrows
///
/// Although [`serde::Deserialize`] is implemented for `&str`, `&[u8]`, and `&std::path::Path`, it
/// is not possible to deserialize to these types from Ruby values. Any attempt to do so will
/// result in an `Err`. The reason is that it cannot be guaranteed the underlying Ruby data will
/// outlive the borrow.
///
/// Use owned equivalents such as `String`, `Vec<u8>`, and [`std::path::PathBuf`] instead.
///
/// #### Strings
///
/// Bad (attempts to deserialize a Ruby `String` into a Rust `&str`):
///
/// ```
/// # use magnus::{eval, Error, Value};
/// # use serde_magnus::deserialize;
/// #
/// # let _cleanup = unsafe { magnus::embed::init() };
/// #
/// let input: Value = eval!("'Hello, world!'")?;
/// let output: Result<&str, Error> = deserialize(&input);
/// assert!(output.is_err());
/// assert_eq!(
///     "TypeError: can't deserialize into borrowed string",
///     output.unwrap_err().to_string()
/// );
/// #
/// # Ok::<(), magnus::Error>(())
/// ```
///
/// Good (deserializes into an owned Rust `String` instead):
///
/// ```
/// # use magnus::{eval, Error, Value};
/// # use serde_magnus::deserialize;
/// #
/// # let _cleanup = unsafe { magnus::embed::init() };
/// #
/// let input: Value = eval!("'Hello, world!'")?;
/// let output: String = deserialize(&input)?;
/// assert_eq!("Hello, world!", output);
/// #
/// # Ok::<(), magnus::Error>(())
/// ```
///
/// #### Bytes
///
/// Bad (attempts to deserialize a Ruby `String` into a Rust byte slice):
///
/// ```
/// # use magnus::{eval, Error, Value};
/// # use serde_magnus::deserialize;
/// #
/// # let _cleanup = unsafe { magnus::embed::init() };
/// #
/// let input: Value = eval!("'☃'")?;
/// let output: Result<&[u8], Error> = deserialize(&input);
/// assert!(output.is_err());
/// assert_eq!(
///     "TypeError: can't deserialize into byte slice",
///     output.unwrap_err().to_string()
/// );
/// #
/// # Ok::<(), magnus::Error>(())
/// ```
///
/// Good (uses `serde_bytes::ByteBuf` to deserialize into an owned `Vec<u8>` instead):
///
/// ```
/// # use magnus::{eval, Error, Value};
/// # use serde_magnus::deserialize;
/// #
/// # let _cleanup = unsafe { magnus::embed::init() };
/// #
/// use serde_bytes::ByteBuf;
///
/// let input: Value = eval!("'☃'")?;
/// let output: ByteBuf = deserialize(&input)?;
/// assert_eq!(vec![226, 152, 131], output.into_vec());
/// #
/// # Ok::<(), magnus::Error>(())
/// ```
pub fn deserialize<'i, Input, Output>(input: Input) -> Result<Output, Error>
where
    Input: Deref<Target = Value>,
    Output: Deserialize<'i>,
{
    Output::deserialize(Deserializer::new(*input)).map_err(Into::into)
}

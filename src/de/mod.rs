mod deserializer;

mod array_deserializer;
mod array_enumerator;
mod enum_deserializer;
mod hash_deserializer;
mod variant_deserializer;

use self::deserializer::Deserializer;

use self::{
    array_deserializer::ArrayDeserializer, enum_deserializer::EnumDeserializer,
    hash_deserializer::HashDeserializer, variant_deserializer::VariantDeserializer,
};

use magnus::{Error, IntoValue, Ruby};
use serde::Deserialize;

/// Deserialize a Ruby [`Value`][`magnus::Value`] to Rust.
///
/// See [`crate::serialize`] for the expected Ruby formats of various Rust types.
///
/// ### Primitive types
///
/// #### Unit type
///
/// ```
/// # let ruby = unsafe { magnus::embed::init() };
/// #
/// use magnus::{eval, Value};
/// use serde_magnus::deserialize;
///
/// let input: Value = eval!(&ruby, "nil")?;
/// let output: () = deserialize(&ruby, input)?;
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
/// # let ruby = unsafe { magnus::embed::init() };
/// #
/// let input: Value = eval!(&ruby, "true")?;
/// let output: bool = deserialize(&ruby, input)?;
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
/// # let ruby = unsafe { magnus::embed::init() };
/// #
/// let input: Value = eval!(&ruby, "1234")?;
/// let output: i64 = deserialize(&ruby, input)?;
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
/// # let ruby = unsafe { magnus::embed::init() };
/// #
/// let input: Value = eval!(&ruby, "3.14")?;
/// let output: f64 = deserialize(&ruby, input)?;
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
/// # let ruby = unsafe { magnus::embed::init() };
/// #
/// let input: Value = eval!(&ruby, r#""Hello, world!""#)?;
/// let output: String = deserialize(&ruby, input)?;
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
/// # let ruby = unsafe { magnus::embed::init() };
/// #
/// let input: Value = eval!(&ruby, "nil")?;
/// let output: Option<i64> = deserialize(&ruby, input)?;
/// assert_eq!(None, output);
///
/// let input: Value = eval!(&ruby, "1234")?;
/// let output: Option<i64> = deserialize(&ruby, input)?;
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
/// # let ruby = unsafe { magnus::embed::init() };
/// #
/// let input: Value = eval!(&ruby, "{ 'Ok' => 1234 }")?;
/// let output: Result<i64, String> = deserialize(&ruby, input)?;
/// assert_eq!(Ok(1234), output);
///
/// let input: Value = eval!(&ruby, "{ 'Err' => 'something went wrong' }")?;
/// let output: Result<i64, String> = deserialize(&ruby, input)?;
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
/// # let ruby = unsafe { magnus::embed::init() };
/// #
/// use serde::Deserialize;
///
/// #[derive(PartialEq, Debug, Deserialize)]
/// struct Foo;
///
/// let input: Value = eval!(&ruby, "nil")?;
/// let output: Foo = deserialize(&ruby, input)?;
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
/// # let ruby = unsafe { magnus::embed::init() };
/// #
/// use serde::Deserialize;
///
/// #[derive(PartialEq, Debug, Deserialize)]
/// struct Foo(u16);
///
/// let input: Value = eval!(&ruby, "1234")?;
/// let output: Foo = deserialize(&ruby, input)?;
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
/// # let ruby = unsafe { magnus::embed::init() };
/// #
/// use serde::Deserialize;
///
/// #[derive(PartialEq, Debug, Deserialize)]
/// struct Foo(u16, bool, String);
///
/// let input: Value = eval!(&ruby, "[123, true, 'Hello, world!']")?;
/// let output: Foo = deserialize(&ruby, input)?;
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
/// # let ruby = unsafe { magnus::embed::init() };
/// #
/// #[derive(PartialEq, Debug, Deserialize)]
/// struct Foo {
///     bar: u16,
///     baz: bool,
///     glorp: String
/// }
///
/// let input: Value = eval!(&ruby, "{ bar: 1234, baz: true, glorp: 'Hello, world!' }")?;
/// let output: Foo = deserialize(&ruby, input)?;
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
/// # let ruby = unsafe { magnus::embed::init() };
/// #
/// # #[derive(PartialEq, Debug, Deserialize)]
/// # enum Foo { Bar }
/// #
/// let input: Value = eval!(&ruby, "'Bar'")?;
/// let output: Foo = deserialize(&ruby, input)?;
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
/// # let ruby = unsafe { magnus::embed::init() };
/// #
/// # #[derive(PartialEq, Debug, Deserialize)]
/// # enum Foo { Baz(u16) }
/// #
/// let input: Value = eval!(&ruby, "{ 'Baz' => 1234 }")?;
/// let output: Foo = deserialize(&ruby, input)?;
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
/// # let ruby = unsafe { magnus::embed::init() };
/// #
/// # #[derive(PartialEq, Debug, Deserialize)]
/// # enum Foo { Glorp(u16, bool, String) }
/// #
/// let input: Value = eval!(&ruby, "{ 'Glorp' => [123, true, 'Hello, world!'] }")?;
/// let output: Foo = deserialize(&ruby, input)?;
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
/// # let ruby = unsafe { magnus::embed::init() };
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
/// let input: Value = eval!(&ruby, r#"
///     {
///       "Quux" => {
///         frob: 1234,
///         wally: true,
///         plugh: "Hello, world!"
///       }
///     }
/// "#)?;
/// let output: Foo = deserialize(&ruby, input)?;
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
/// # let ruby = unsafe { magnus::embed::init() };
/// #
/// let input: Value = eval!(&ruby, "[123, true, 'Hello, world!']")?;
/// let output: (i16, bool, String) = deserialize(&ruby, input)?;
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
/// # let ruby = unsafe { magnus::embed::init() };
/// #
/// let input: Value = eval!(&ruby, "[123, 456, 789]")?;
/// let output: [i64; 3] = deserialize(&ruby, input)?;
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
/// # let ruby = unsafe { magnus::embed::init() };
/// #
/// let input: Value = eval!(&ruby, "[123, 456, 789]")?;
/// let output: Vec<u64> = deserialize(&ruby, input)?;
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
/// # let ruby = unsafe { magnus::embed::init() };
/// #
/// use std::collections::HashMap;
///
/// let input: Value = eval!(&ruby, r#"
///     {
///       "yes" => "no",
///       "stop" => "go",
///       "high" => "low",
///       "goodbye" => "hello"
///     }
/// "#)?;
/// let output: HashMap<String, String> = deserialize(&ruby, input)?;
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
/// # let ruby = unsafe { magnus::embed::init() };
/// #
/// let input: Value = eval!(&ruby, "'Hello, world!'")?;
/// let output: Result<&str, Error> = deserialize(&ruby, input);
/// assert!(output.is_err());
/// assert_eq!(
///    r#"TypeError: invalid type: expected a borrowed string, got string "Hello, world!""#,
///    output.unwrap_err().to_string()
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
/// # let ruby = unsafe { magnus::embed::init() };
/// #
/// let input: Value = eval!(&ruby, "'Hello, world!'")?;
/// let output: String = deserialize(&ruby, input)?;
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
/// # let ruby = unsafe { magnus::embed::init() };
/// #
/// let input: Value = eval!(&ruby, "'☃'")?;
/// let output: Result<&[u8], Error> = deserialize(&ruby, input);
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
/// # let ruby = unsafe { magnus::embed::init() };
/// #
/// use serde_bytes::ByteBuf;
///
/// let input: Value = eval!(&ruby, "'☃'")?;
/// let output: ByteBuf = deserialize(&ruby, input)?;
/// assert_eq!(vec![226, 152, 131], output.into_vec());
/// #
/// # Ok::<(), magnus::Error>(())
/// ```
pub fn deserialize<'i, Input, Output>(ruby: &Ruby, input: Input) -> Result<Output, Error>
where
    Input: IntoValue,
    Output: Deserialize<'i>,
{
    Output::deserialize(Deserializer::new(ruby, input.into_value_with(ruby))).map_err(Into::into)
}

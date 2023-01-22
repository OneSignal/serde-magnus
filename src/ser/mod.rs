mod serializer;

mod enums;
mod map_serializer;
mod seq_serializer;
mod struct_serializer;
mod struct_variant_serializer;
mod tuple_variant_serializer;

use self::serializer::Serializer;

use self::{
    map_serializer::MapSerializer, seq_serializer::SeqSerializer,
    struct_serializer::StructSerializer, struct_variant_serializer::StructVariantSerializer,
    tuple_variant_serializer::TupleVariantSerializer,
};

use magnus::{Error, TryConvert};
use serde::Serialize;

/// Serialize Rust data to a Ruby [`Value`][`magnus::Value`].
///
/// Primitive types are converted as follows:
///
/// | Rust type                 | Ruby value                          |
/// |---------------------------|-------------------------------------|
/// | `()`                      | `nil`                               |
/// | `bool`                    | `true` or `false`                   |
/// | `i8`, `i16`, `i32`, `i64` | An `Integer`                        |
/// | `u8`, `u16`, `u32`, `u64` | An `Integer`                        |
/// | `f32` or `f64`            | A `Float`                           |
/// | `char`                    | A `String` with UTF-8 encoding      |
/// | `&str`                    | A `String` with UTF-8 encoding      |
/// | `String`                  | A `String` with UTF-8 encoding      |
/// | `&[u8]`                   | A `String` with ASCII-8BIT encoding |
///
/// ```
/// use magnus::{eval, Value};
/// use serde_magnus::serialize;
/// # let _cleanup = unsafe { magnus::embed::init() };
///
/// let output: Value = serialize(&1234)?;
/// assert!(eval!("output == 1234", output)?);
///
/// let output: Value = serialize("Hello, world!")?;
/// assert!(eval!("output == 'Hello, world!'", output)?);
///
/// # Ok::<(), magnus::Error>(())
/// ```
///
/// ### `Option`
///
/// `None` is converted to `nil`. `Some` is unwrapped.
///
/// ```
/// # use magnus::{eval, Value};
/// # use serde_magnus::serialize;
/// # let _cleanup = unsafe { magnus::embed::init() };
/// let input: Option<u64> = None;
/// let output: Value = serialize(&input)?;
/// assert!(eval!("output == nil", output)?);
///
/// let input: Option<u64> = Some(1234);
/// let output: Value = serialize(&input)?;
/// assert!(eval!("output == 1234", output)?);
///
/// # Ok::<(), magnus::Error>(())
/// ```
///
/// ### `Result`
///
/// A `Result` is converted to a `Hash` with one key, `"Ok"` or `"Err"`.
///
/// ```
/// # use magnus::{eval, Value};
/// # use serde_magnus::serialize;
/// # let _cleanup = unsafe { magnus::embed::init() };
/// let input: Result<u64, &str> = Ok(1234);
/// let output: Value = serialize(&input)?;
/// assert!(eval!("output == { 'Ok' => 1234 }", output)?);
///
/// let input: Result<u64, &str> = Err("something went wrong");
/// let output: Value = serialize(&input)?;
/// assert!(eval!("output == { 'Err' => 'something went wrong' }", output)?);
///
/// # Ok::<(), magnus::Error>(())
/// ```
///
/// ### Structs
///
/// A unit struct is converted to `nil`.
///
/// ```
/// # use magnus::{eval, Value};
/// # use serde_magnus::serialize;
/// # let _cleanup = unsafe { magnus::embed::init() };
/// use serde::Serialize;
///
/// #[derive(Serialize)]
/// struct Foo;
///
/// let output: Value = serialize(&Foo)?;
/// assert!(eval!("output == nil", output)?);
///
/// # Ok::<(), magnus::Error>(())
/// ```
///
/// A newtype struct is unwrapped.
///
/// ```
/// # use magnus::{eval, Value};
/// # use serde_magnus::serialize;
/// # let _cleanup = unsafe { magnus::embed::init() };
/// # use serde::Serialize;
/// #[derive(Serialize)]
/// struct Foo(u16);
///
/// let input = Foo(1234);
/// let output: Value = serialize(&input)?;
/// assert!(eval!("output == 1234", output)?);
///
/// # Ok::<(), magnus::Error>(())
/// ```
///
/// A tuple struct is converted to an `Array` of its fields.
///
/// ```
/// # use magnus::{eval, Value};
/// # use serde_magnus::serialize;
/// # let _cleanup = unsafe { magnus::embed::init() };
/// # use serde::Serialize;
/// #[derive(Serialize)]
/// struct Foo<'a>(u16, bool, &'a str);
///
/// let input = Foo(1234, false, "Hello, world!");
/// let output: Value = serialize(&input)?;
/// assert!(eval!("output == [1234, false, 'Hello, world!']", output)?);
///
/// # Ok::<(), magnus::Error>(())
/// ```
///
/// A struct with named fields is converted to a `Hash` with symbol keys.
///
/// ```
/// # use magnus::{eval, Value};
/// # use serde_magnus::serialize;
/// # let _cleanup = unsafe { magnus::embed::init() };
/// # use serde::Serialize;
/// #[derive(Serialize)]
/// struct Foo<'a> {
///     bar: u16,
///     baz: bool,
///     glorp: &'a str
/// }
///
/// let input = Foo { bar: 1234, baz: false, glorp: "Hello, world!" };
/// let output: Value = serialize(&input)?;
/// assert!(eval!(
///     r#"
///     output == {
///       bar: 1234,
///       baz: false,
///       glorp: "Hello, world!"
///     }
///     "#,
///     output
/// )?);
///
/// # Ok::<(), magnus::Error>(())
/// ```
///
/// ### Enums
///
/// ```
/// # use serde::Serialize;
/// #[derive(Serialize)]
/// enum Foo<'a> {
///     Bar,
///     Baz(u16),
///     Glorp(u16, bool, &'a str),
///     Quux {
///         frob: u16,
///         wally: bool,
///         plugh: &'a str
///     }
/// }
/// ```
///
/// A unit enum variant is converted to the name of the variant as a `String`.
///
/// ```
/// # use magnus::{eval, Value};
/// # use serde_magnus::serialize;
/// # let _cleanup = unsafe { magnus::embed::init() };
/// # use serde::Serialize;
/// # #[derive(Serialize)]
/// # enum Foo<'a> {
/// #     Bar,
/// #     Baz(u16),
/// #     Glorp(u16, bool, &'a str),
/// #     Quux {
/// #         frob: u16,
/// #         wally: bool,
/// #         plugh: &'a str
/// #     }
/// # }
/// let output: Value = serialize(&Foo::Bar)?;
/// assert!(eval!("output == 'Bar'", output)?);
///
/// # Ok::<(), magnus::Error>(())
/// ```
///
/// All other types of enum variants are converted to a `Hash` with one key: the name of the
/// variant as a `String`.
///
/// For a newtype enum variant, the value keyed by the variant's name is its unwrapped value.
///
/// ```
/// # use magnus::{eval, Value};
/// # use serde_magnus::serialize;
/// # let _cleanup = unsafe { magnus::embed::init() };
/// # use serde::Serialize;
/// # #[derive(Serialize)]
/// # enum Foo<'a> {
/// #     Bar,
/// #     Baz(u16),
/// #     Glorp(u16, bool, &'a str),
/// #     Quux {
/// #         frob: u16,
/// #         wally: bool,
/// #         plugh: &'a str
/// #     }
/// # }
/// let input = Foo::Baz(1234);
/// let output: Value = serialize(&input)?;
/// assert!(eval!("output == { 'Baz' => 1234 }", output)?);
///
/// # Ok::<(), magnus::Error>(())
/// ```
///
/// For a tuple enum variant, the value keyed by the variant's name is an `Array`.
///
/// ```
/// # use magnus::{eval, Value};
/// # use serde_magnus::serialize;
/// # let _cleanup = unsafe { magnus::embed::init() };
/// # use serde::Serialize;
/// # #[derive(Serialize)]
/// # enum Foo<'a> {
/// #     Bar,
/// #     Baz(u16),
/// #     Glorp(u16, bool, &'a str),
/// #     Quux {
/// #         frob: u16,
/// #         wally: bool,
/// #         plugh: &'a str
/// #     }
/// # }
/// let input = Foo::Glorp(1234, false, "Hello, world!");
/// let output: Value = serialize(&input)?;
/// assert!(eval!("output == { 'Glorp' => [1234, false, 'Hello, world!'] }", output)?);
///
/// # Ok::<(), magnus::Error>(())
/// ```
///
/// For a struct enum variant, the value keyed by the variant name is a `Hash` with `Symbol` keys.
///
/// ```
/// # use magnus::{eval, Value};
/// # use serde_magnus::serialize;
/// # let _cleanup = unsafe { magnus::embed::init() };
/// # use serde::Serialize;
/// # #[derive(Serialize)]
/// # enum Foo<'a> {
/// #     Bar,
/// #     Baz(u16),
/// #     Glorp(u16, bool, &'a str),
/// #     Quux {
/// #         frob: u16,
/// #         wally: bool,
/// #         plugh: &'a str
/// #     }
/// # }
/// let input = Foo::Quux { frob: 1234, wally: false, plugh: "Hello, world!" };
/// let output: Value = serialize(&input)?;
/// assert!(eval!(
///     r#"
///     output == {
///       "Quux" => {
///         frob: 1234,
///         wally: false,
///         plugh: "Hello, world!"
///       }
///     }
///     "#,
///     output
/// )?);
///
/// # Ok::<(), magnus::Error>(())
/// ```
///
/// ### Compound types
///
/// A compound type such as a tuple `(T1, T2, T3, ...)`, array `[T; N]`, or slice `[T]` is
/// converted to an `Array`.
///
/// ```
/// # use magnus::{eval, Value};
/// # use serde_magnus::serialize;
/// # let _cleanup = unsafe { magnus::embed::init() };
/// let input = (123, false, "Hello, world!");
/// let output: Value = serialize(&input)?;
/// assert!(eval!("output == [123, false, 'Hello, world!']", output)?);
///
/// let input = [123, 456, 789];
/// let output: Value = serialize(&input)?;
/// assert!(eval!("output == [123, 456, 789]", output)?);
///
/// # Ok::<(), magnus::Error>(())
/// ```
///
/// ### Collections
///
/// A sequence such as a `Vec`, `LinkedList`, or `HashSet` is converted to an `Array`.
///
/// ```
/// # use magnus::{eval, Value};
/// # use serde_magnus::serialize;
/// # let _cleanup = unsafe { magnus::embed::init() };
/// let input = vec![123, 456, 789];
/// let output: Value = serialize(&input)?;
/// assert!(eval!("output == [123, 456, 789]", output)?);
///
/// # Ok::<(), magnus::Error>(())
/// ```
///
/// A map such as a `HashMap` or `BTreeMap` is converted to a `Hash`.
///
/// ```
/// # use magnus::{eval, Value};
/// # use serde_magnus::serialize;
/// # let _cleanup = unsafe { magnus::embed::init() };
/// use std::collections::HashMap;
///
/// let mut input = HashMap::new();
/// input.insert("yes", "no");
/// input.insert("stop", "go");
/// input.insert("high", "low");
/// input.insert("goodbye", "hello");
///
/// let output: Value = serialize(&input)?;
/// assert!(eval!(
///     r#"
///     output == {
///       "yes"     => "no",
///       "stop"    => "go",
///       "high"    => "low",
///       "goodbye" => "hello"
///     }
///     "#,
///     output
/// )?);
///
/// # Ok::<(), magnus::Error>(())
/// ```
pub fn serialize<Input, Output>(input: &Input) -> Result<Output, Error>
where
    Input: Serialize + ?Sized,
    Output: TryConvert,
{
    input
        .serialize(Serializer)?
        .try_convert()
        .map_err(Into::into)
}

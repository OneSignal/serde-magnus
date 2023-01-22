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
/// The most basic Rust types are converted as follows:
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
/// A unit enum variant (`A::Z` in the example code below) is converted to the name of the
/// variant as a `String` (`"Z"`).
///
/// All other types of enum variants (`A::Y`, `A::X`, `A::W`) are converted to a `Hash` with one
/// key: the name of the variant as a `String` (`"Y"`', `"X"`, `"W"`).
///
/// For a newtype enum variant (`A::Y`), the value keyed by the variant's name is its unwrapped
/// value.
///
/// For a tuple enum variant (`A::X`), the value keyed by the variant name is an `Array` containing
/// the variant's fields.
///
/// For a struct enum variant (`A::W`), the value keyed by the variant name is a `Hash` with the
/// variant's field names as `Symbol` keys.
///
/// ```
/// # use magnus::{eval, Value};
/// # use serde::Serialize;
/// # use serde_magnus::serialize;
/// # let _cleanup = unsafe { magnus::embed::init() };
/// #[derive(Serialize)]
/// enum A {
///     Z,
///     Y(u16),
///     X(u16, bool, Box<A>),
///     W {
///         foo: u16,
///         bar: bool,
///         baz: Box<A>
///     }
/// }
///
/// let output: Value = serialize(&A::Z)?;
/// assert!(eval!(r#"output == "Z""#, output)?);
///
/// let input = A::Y(1234);
/// let output: Value = serialize(&input)?;
/// assert!(eval!(r#"output == { "Y" => 1234 }"#, output)?);
///
/// let input = A::X(
///     1234,
///     false,
///     Box::new(A::Y(5678))
/// );
/// let output: Value = serialize(&input)?;
/// assert!(eval!(
///     r#"
///     output == {
///       "X" => [
///         1234,
///         false,
///         { "Y" => 5678 }
///       ]
///     }
///     "#,
///     output
/// )?);
///
/// let input = A::W {
///     foo: 1234,
///     bar: false,
///     baz: Box::new(A::Y(5678))
/// };
/// let output: Value = serialize(&input)?;
/// assert!(eval!(
///     r#"
///     output == {
///       "W" => {
///         foo: 1234,
///         bar: false,
///         baz: { "Y" => 5678 }
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
/// A compound type such as a tuple `(T1, T2, T3, ...)` or array `[T]` is converted to an `Array`.
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
/// let input = HashMap::from([
///     ("yes", "no"),
///     ("stop", "go"),
///     ("high", "low"),
///     ("goodbye", "hello")
/// ]);
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

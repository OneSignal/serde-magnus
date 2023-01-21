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
/// use magnus::{Integer, RString};
/// use serde_magnus::serialize;
/// # let _cleanup = unsafe { magnus::embed::init() };
///
/// let output: Integer = serialize(&1234)?;
/// assert_eq!(1234, output.to_u64()?);
///
/// let output: RString = serialize("Hello, world!")?;
/// assert_eq!("Hello, world!", output.to_string()?);
///
/// # Ok::<(), magnus::Error>(())
/// ```
///
/// ### `Option`
///
/// `None` is converted to `nil`.
///
/// `Some` is unwrapped and its content value is recursively serialized.
///
/// ```
/// use magnus::{Integer, Value};
/// use serde_magnus::serialize;
/// # let _cleanup = unsafe { magnus::embed::init() };
///
/// let input: Option<u64> = None;
/// let output: Value = serialize(&input)?;
/// assert!(output.is_nil());
///
/// let input: Option<u64> = Some(1234);
/// let output: Integer = serialize(&input)?;
/// assert_eq!(1234, output.to_u64()?);
///
/// # Ok::<(), magnus::Error>(())
/// ```
///
/// ### Structs
///
/// A unit struct (`A` in the example code below) is converted to `nil`.
///
/// A newtype struct (`B`) is unwrapped. Its value is recursively serialized.
///
/// A tuple struct (`C`) is converted to an `Array`. The `Array` contains the struct's fields,
/// recursively serialized.
///
/// A struct with named fields (`D`) is converted to a `Hash` with the field names as `Symbol`
/// keys. The field values are recursively serialized.
///
/// ```
/// use magnus::{Integer, RArray, RHash, Symbol, Value};
/// use serde::Serialize;
/// use serde_magnus::serialize;
/// # let _cleanup = unsafe { magnus::embed::init() };
///
/// #[derive(Serialize)]
/// struct A;
///
/// #[derive(Serialize)]
/// struct B(u16);
///
/// #[derive(Serialize)]
/// struct C(u16, bool, B);
///
/// #[derive(Serialize)]
/// struct D {
///     foo: u16,
///     bar: bool,
///     baz: B
/// }
///
/// let output: Value = serialize(&A)?;
/// assert!(output.is_nil());
///
/// let output: Integer = serialize(&B(1234))?;
/// assert_eq!(1234, output.to_u64()?);
///
/// let input = C(1234, false, B(5678));
/// let output: RArray = serialize(&input)?;
/// assert_eq!(3, output.len());
/// assert_eq!(1234, output.entry::<u16>(0)?);
/// assert_eq!(false, output.entry::<bool>(1)?);
/// assert_eq!(5678, output.entry::<u16>(2)?);
///
/// let input = D { foo: 1234, bar: false, baz: B(5678) };
/// let output: RHash = serialize(&input)?;
/// assert_eq!(1234, output.lookup::<_, u16>(Symbol::new("foo"))?);
/// assert_eq!(false, output.lookup::<_, bool>(Symbol::new("bar"))?);
/// assert_eq!(5678, output.lookup::<_, u16>(Symbol::new("baz"))?);
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
/// For a newtype enum variant (`A::Y`), the value keyed by the variant name is the variant's value
/// recursively serialized.
///
/// For a tuple enum variant (`A::X`), the value keyed by the variant name is an `Array` containing
/// the variant's fields recursively serialized.
///
/// For a struct enum variant (`A::W`), the value keyed by the variant name is a `Hash` with the
/// variant's field names as `Symbol` keys. The field values are recursively serialized.
///
/// ```
/// use magnus::{Integer, RArray, RHash, RString, Symbol, Value};
/// use serde::Serialize;
/// use serde_magnus::serialize;
/// # let _cleanup = unsafe { magnus::embed::init() };
///
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
/// let output: RString = serialize(&A::Z)?;
/// assert_eq!("Z", output.to_string()?);
///
/// let input = A::Y(1234);
/// let output: RHash = serialize(&input)?;
/// assert_eq!(1, output.len());
/// assert_eq!(1234, output.lookup::<_, u16>("Y")?);
///
/// let input = A::X(
///     1234,
///     false,
///     Box::new(A::Y(5678))
/// );
/// let output: RHash = serialize(&input)?;
/// let value: RArray = output.lookup("X")?;
/// assert_eq!(3, value.len());
/// assert_eq!(1234, value.entry::<u16>(0)?);
/// assert_eq!(false, value.entry::<bool>(1)?);
/// let value: RHash = value.entry(2)?;
/// assert_eq!(5678, value.lookup::<_, u16>("Y")?);
///
/// let input = A::W {
///     foo: 1234,
///     bar: false,
///     baz: Box::new(A::Y(5678))
/// };
/// let output: RHash = serialize(&input)?;
/// let value: RHash = output.lookup("W")?;
/// assert_eq!(1234, value.lookup::<_, u16>(Symbol::new("foo"))?);
/// assert_eq!(false, value.lookup::<_, bool>(Symbol::new("bar"))?);
/// let value: RHash = value.lookup(Symbol::new("baz"))?;
/// assert_eq!(5678, value.lookup::<_, u16>("Y")?);
///
/// # Ok::<(), magnus::Error>(())
/// ```
///
/// ### Compound types
///
/// A compound type such as a tuple `(T1, T2, T3, ...)` or array `[T]` is converted to an `Array`.
/// Its members are recursively serialized.
///
/// ```
/// use magnus::{Integer, RArray};
/// use serde_magnus::serialize;
/// # let _cleanup = unsafe { magnus::embed::init() };
///
/// let input = (123, false, "Hello, world!");
/// let output: RArray = serialize(&input)?;
/// assert_eq!(3, output.len());
/// assert_eq!(123, output.entry::<i64>(0)?);
/// assert_eq!(false, output.entry::<bool>(1)?);
/// assert_eq!("Hello, world!", output.entry::<String>(2)?);
///
/// let input = [123, 456, 789];
/// let output: RArray = serialize(&input)?;
/// assert_eq!(3, output.len());
/// assert_eq!(123, output.entry::<i64>(0)?);
/// assert_eq!(456, output.entry::<i64>(1)?);
/// assert_eq!(789, output.entry::<i64>(2)?);
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

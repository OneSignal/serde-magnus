mod serializer;

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

/// Serialize Rust data to a Ruby [`Value`].
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
/// let magic: Integer = serialize(&1234).unwrap();
/// assert_eq!(1234, magic.to_u64().unwrap());
///
/// let greeting: RString = serialize("Hello, world!").unwrap();
/// assert_eq!("Hello, world!", greeting.to_string().unwrap());
/// ```
///
/// ### `Option`
///
/// `None` is converted to `nil`. For an `Option<T>`, `Some` is unwrapped and its `T` value is
/// recursively serialized.
///
/// ```
/// use magnus::{Integer, Value};
/// use serde_magnus::serialize;
/// # let _cleanup = unsafe { magnus::embed::init() };
///
/// let magic: Option<u64> = None;
/// let magic: Value = serialize(&magic).unwrap();
/// assert!(magic.is_nil());
///
/// let magic: Option<u64> = Some(1234);
/// let magic: Integer = serialize(&magic).unwrap();
/// assert_eq!(1234, magic.to_u64().unwrap());
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
/// let a: Value = serialize(&A).unwrap();
/// assert!(a.is_nil());
///
/// let b: Integer = serialize(&B(1234)).unwrap();
/// assert_eq!(1234, b.to_u64().unwrap());
///
/// let c: RArray = serialize(&C(1234, false, B(5678))).unwrap();
/// assert_eq!(3, c.len());
/// assert_eq!(1234, c.entry(0).unwrap());
/// assert_eq!(false, c.entry(1).unwrap());
/// assert_eq!(5678, c.entry(2).unwrap());
///
/// let d: RHash = serialize(&D { foo: 1234, bar: false, baz: B(5678) }).unwrap();
/// assert_eq!(1234, d.lookup(Symbol::new("foo")).unwrap());
/// assert_eq!(false, d.lookup(Symbol::new("bar")).unwrap());
/// assert_eq!(5678, d.lookup(Symbol::new("baz")).unwrap());
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
/// let a: RString = serialize(&A::Z).unwrap();
/// assert_eq!("Z", a.to_string().unwrap());
///
/// let a: RHash = serialize(&A::Y(1234)).unwrap();
/// assert_eq!(1, a.len());
/// assert_eq!(1234, a.lookup("Y").unwrap());
///
/// let a: RHash = serialize(&A::X(1234, false, Box::new(A::Y(5678)))).unwrap();
/// let ax: RArray = a.lookup("X").unwrap();
/// assert_eq!(3, ax.len());
/// assert_eq!(1234, ax.entry(0).unwrap());
/// assert_eq!(false, ax.entry(1).unwrap());
/// assert_eq!(5678, ax.entry::<RHash>(2).unwrap().lookup("Y").unwrap());
///
/// let a: RHash = serialize(&A::W { foo: 1234, bar: false, baz: Box::new(A::Y(5678)) }).unwrap();
/// let w: RHash = a.lookup("W").unwrap();
/// assert_eq!(1234, w.lookup(Symbol::new("foo")).unwrap());
/// assert_eq!(false, w.lookup(Symbol::new("bar")).unwrap());
/// let baz: RHash = w.lookup(Symbol::new("baz")).unwrap();
/// assert_eq!(5678, baz.lookup("Y").unwrap());
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

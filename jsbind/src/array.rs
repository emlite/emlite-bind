use crate::any::Any;
use crate::sequence::Sequence;
use crate::utils::bind;
use emlite::FromVal;

/// Generic WebIDL typed array ([spec § 2.7]).
///
/// This is nothing more than a [`Sequence<T>`] with a canonical name that
/// matches the WebIDL grammar.  All sequence helpers (`len`, `push`, `get`,
/// `iter`, …) are therefore immediately available.
///
/// [spec § 2.7]: https://webidl.spec.whatwg.org/#idl-typedarray
pub type TypedArray<T> = Sequence<T>;

/// An immutable view of a typed array (WebIDL frozen array).
///
/// The alias emphasises intent; mutating methods like `push` will still exist
/// at compile‑time, so it is the embedding API’s responsibility to enforce
/// read‑only semantics when a `FrozenArray<T>` is handed out.
pub type FrozenArray<T> = TypedArray<T>;

/// A typed array that notifies observers when mutated.
///
/// This behaves exactly like `TypedArray<T>` in Rust—notification is a JS‑side
/// concern handled by the underlying `Proxy` or equivalent construct.
pub type ObservableArray<T> = TypedArray<T>;

/// Concrete aliases for the built‑in ECMAScript numeric array types.
pub type Uint8Array = TypedArray<u8>;
pub type Int8Array = TypedArray<i8>;
pub type Uint32Array = TypedArray<u32>;
pub type Int32Array = TypedArray<i32>;
pub type Float32Array = TypedArray<f32>;
pub type Float64Array = TypedArray<f64>;

/// Heterogeneous JavaScript `Array` (equivalent to `Vec<JsValue>`).
///
/// Using `Sequence<Any>` means every element is an arbitrary JS value that can
/// be converted on demand with `as_::<T>()`.
pub type Array = Sequence<Any>;

/// A raw, fixed‑length buffer of bytes as defined by the ECMAScript spec.
#[derive(Clone, Debug)]
pub struct ArrayBuffer {
    /// Underlying JavaScript value (always a `Promise` object in JS land).
    inner: emlite::Val,
}

impl ArrayBuffer {
    /// Construct a new zero‑initialised buffer of `byte_len` bytes.
    /// Equivalent to `new ArrayBuffer(byte_len)` in JavaScript.
    pub fn new(byte_len: usize) -> Self {
        let ctor = emlite::Val::global("ArrayBuffer");
        let v = ctor.new(&[byte_len.into()]);
        Self::from_val(&v)
    }

    /// Total size of the buffer in bytes (`buffer.byteLength`).
    #[doc(alias = "byteLength")]
    pub fn byte_length(&self) -> usize {
        self.inner.get("byteLength").as_::<usize>()
    }

    /// `buffer.slice(begin, end)` – creates a new `ArrayBuffer` that shares
    /// memory with the original.  If `end` is `None`, the slice goes to
    /// the end of the buffer.
    pub fn slice(&self, begin: usize, end: Option<usize>) -> Self {
        match end {
            Some(e) => self.inner.call("slice", &[begin.into(), e.into()]),
            None => self.inner.call("slice", &[begin.into()]),
        }
        .as_::<Self>()
    }

    pub fn is_view(buf: &Any) -> bool {
        emlite::Val::global("ArrayBuffer")
            .call("isView", &[buf.clone()])
            .as_::<bool>()
    }

    pub fn resizable(&self) -> bool {
        self.inner.get("resizable").as_::<bool>()
    }
}

bind!(ArrayBuffer);

/// Explicit byte‑order flag for `DataView` getters / setters.
///
/// ECMAScript treats the `littleEndian` parameter as a boolean.  We expose an
/// enum so that call‑sites are self‑documenting – passing `Endian::Little`
/// reads more clearly than `true`.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Endian {
    /// Most‑significant byte first (`false` in JS).
    Big,
    /// Least‑significant byte first (`true` in JS).
    Little,
}

impl Endian {
    /// Convert the flag to the string that our JS shim understands.
    pub fn to_str(&self) -> &'static str {
        match *self {
            Endian::Big => "bigEndian",
            Endian::Little => "littleEndian",
        }
    }
}

/// Provides a configurable view on top of an [`ArrayBuffer`].
///
/// All accessor methods follow the spec signature `(byteOffset, [value],
/// littleEndian)` so host code must always pass an [`Endian`] flag to remove
/// any ambiguity about byte order.
#[derive(Clone, Debug)]
pub struct DataView {
    inner: emlite::Val,
}

/// Internal helper to expand repetitive getter / setter boilerplate for every
/// numeric width.
///
/// The macro is deliberately not exported; end users interact only with
/// the generated methods on [`DataView`].
macro_rules! rw {
    ($get:ident, $set:ident, $ty:ty) => {
        #[doc = concat!("Reads a `", stringify!($ty), "` at byteOffset with the specified endianness.")]
        pub fn $get(&self, byte_offset: usize, endian: Option<Endian>) -> $ty {
            self.inner
                .call(
                    concat!("get", stringify!($get)),
                    &[byte_offset.into(),  endian.unwrap_or(Endian::Little).to_str().into()],
                )
                .as_::<$ty>()
        }

        #[doc = concat!("Writes value as `", stringify!($ty), "` at byteOffset using the specified endianness.")]
        pub fn $set(&self, byte_offset: usize, value: $ty, endian: Option<Endian>) {
            self.inner.call(
                concat!("set", stringify!($get)),
                &[byte_offset.into(), value.into(), endian.unwrap_or(Endian::Little).to_str().into()],
            );
        }
    };
}

impl DataView {
    /// `new DataView(buffer, byteOffset, byteLength?)`
    ///
    /// `buffer`   – the source `ArrayBuffer`.
    /// `offset`   – start position, in bytes, from the beginning of the buffer.
    /// `len`      – optional byte length for the view; `None` means
    ///                    *"to the end of the buffer"*.
    pub fn new(buffer: &ArrayBuffer, offset: usize, len: Option<usize>) -> Self {
        let ctor = emlite::Val::global("DataView");
        let mut args = vec![buffer.clone().into(), offset.into()];
        if let Some(l) = len {
            args.push(l.into());
        }
        let v = ctor.new(&args);
        Self::from_val(&v)
    }

    /// Size of the view in bytes (`dataView.byteLength`).
    #[inline]
    #[doc(alias = "byteLength")]
    pub fn byte_length(&self) -> usize {
        self.inner.get("byteLength").as_::<usize>()
    }

    rw!(uint8, set_uint8, u8);
    rw!(int8, set_int8, i8);
    rw!(uint16, set_uint16, u16);
    rw!(int16, set_int16, i16);
    rw!(uint32, set_uint32, u32);
    rw!(int32, set_int32, i32);
    rw!(float32, set_float32, f32);
    rw!(float64, set_float64, f64);
}

bind!(DataView);

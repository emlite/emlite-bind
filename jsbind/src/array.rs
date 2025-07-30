use crate::any::Any;
use crate::utils::*;
use alloc::vec;
use alloc::vec::Vec;
use core::marker::PhantomData;
use core::ops::{Deref, DerefMut};
use emlite::FromVal;

/// Parameterised wrapper around a JavaScript array object. No concrete JS type, but needed for WebIDL
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct TypedArray<T> {
    inner: emlite::Val,
    phantom: PhantomData<T>,
}

impl<T> emlite::FromVal for TypedArray<T> {
    fn from_val(v: &emlite::Val) -> Self {
        Self {
            inner: v.clone(),
            phantom: PhantomData,
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}

impl<T> From<TypedArray<T>> for emlite::Val {
    fn from(x: TypedArray<T>) -> emlite::Val {
        let handle = x.inner.as_handle();
        core::mem::forget(x);
        emlite::Val::take_ownership(handle)
    }
}

impl<T> From<&TypedArray<T>> for emlite::Val {
    fn from(x: &TypedArray<T>) -> emlite::Val {
        x.inner.clone()
    }
}

impl<T> Deref for TypedArray<T> {
    type Target = emlite::Val;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T> DerefMut for TypedArray<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl<T> AsRef<emlite::Val> for TypedArray<T> {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}

impl<T> AsMut<emlite::Val> for TypedArray<T> {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}

impl<T> TypedArray<T> {
    /// Construct a new JS array from a Rust slice, pushing each element
    /// through `Into<Val>`.
    pub fn new_from_slice(slice: &[T]) -> Self
    where
        T: Clone + Into<emlite::Val>,
    {
        let arr = emlite::Val::array();
        for v in slice {
            arr.call("push", &[v.clone().into()]);
        }
        Self {
            inner: arr,
            phantom: PhantomData,
        }
    }

    /// Number of elements (`array.length`).
    #[inline]
    pub fn len(&self) -> usize {
        self.inner.get("length").as_::<usize>()
    }
    /// True when `len() == 0`.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Push a single element to the end of the array.
    pub fn push(&mut self, value: T)
    where
        T: Into<emlite::Val>,
    {
        self.inner.call("push", &[value.into()]);
    }

    /// Return a copy of the element at `idx` converted to `T`.
    pub fn get(&self, idx: usize) -> Option<T>
    where
        T: FromVal,
    {
        let v = self.inner.get(idx);
        if v.is_undefined() {
            None
        } else {
            Some(v.as_::<T>())
        }
    }

    /// Returns whether a value exists in the TypedArray.
    pub fn has(&self, val: T) -> bool
    where
        emlite::Val: From<T>,
    {
        self.inner.has(val)
    }

    pub fn set(&self, idx: usize, val: T)
    where
        T: FromVal,
        emlite::Val: From<T>,
    {
        self.inner.set(idx, val);
    }

    /// Returns a Rust Vec from a TypedArray
    pub fn to_vec(&self) -> Vec<T>
    where
        T: FromVal,
    {
        self.iter().collect()
    }
}

pub struct TypedArrayIter<'a, T> {
    parent: &'a TypedArray<T>,
    idx: usize,
    len: usize,
}

impl<T> Iterator for TypedArrayIter<'_, T>
where
    T: FromVal,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.idx < self.len {
            let v = self.parent.get(self.idx);
            self.idx += 1;
            v
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remain = self.len - self.idx;
        (remain, Some(remain))
    }
}

impl<'a, T> IntoIterator for &'a TypedArray<T>
where
    T: FromVal,
{
    type Item = T;
    type IntoIter = TypedArrayIter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        TypedArrayIter {
            parent: self,
            idx: 0,
            len: self.len(),
        }
    }
}

impl<T> TypedArray<T>
where
    T: FromVal,
{
    /// Return an iterator over owned elements.
    pub fn iter(&self) -> TypedArrayIter<'_, T> {
        self.into_iter()
    }
}

impl<T: Clone> From<Vec<T>> for TypedArray<T>
where
    emlite::Val: From<T>,
{
    #[inline]
    fn from(buf: Vec<T>) -> Self {
        // One copy from Wasm linear memory → JS Uint8Array.
        Self::new_from_slice(&buf)
    }
}

impl<T: Clone> From<&[T]> for TypedArray<T>
where
    emlite::Val: From<T>,
{
    #[inline]
    fn from(slice: &[T]) -> Self {
        Self::new_from_slice(slice)
    }
}

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

crate::utils::impl_dyn_cast!(Uint8Array, "Uint8Array");
crate::utils::impl_dyn_cast!(Int8Array, "Int8Array");
crate::utils::impl_dyn_cast!(Uint32Array, "Uint32Array");
crate::utils::impl_dyn_cast!(Int32Array, "Int32Array");
crate::utils::impl_dyn_cast!(Float32Array, "Float32Array");
crate::utils::impl_dyn_cast!(Float64Array, "Float64Array");

/// Heterogeneous JavaScript `Array` (equivalent to `Vec<JsValue>`).
///
/// Using `TypedArray<Any>` means every element is an arbitrary JS value that can
/// be converted on demand with `as_::<T>()`.
pub type Array = TypedArray<Any>;

crate::utils::impl_dyn_cast!(Array, "Array");

/// A raw, fixed‑length buffer of bytes as defined by the ECMAScript spec.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct ArrayBuffer {
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

    /// Static `ArrayBuffer.isView`
    pub fn is_view(buf: &Any) -> bool {
        emlite::Val::global("ArrayBuffer")
            .call("isView", &[buf.clone()])
            .as_::<bool>()
    }

    /// Returns whether the ArrayBuffer is resizable
    pub fn resizable(&self) -> bool {
        self.inner.get("resizable").as_::<bool>()
    }
}

bind!(ArrayBuffer);
impl_dyn_cast!(ArrayBuffer);

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

/// Provides a configurable view on top of an `ArrayBuffer`.
///
/// All accessor methods follow the spec signature `(byteOffset, [value],
/// littleEndian)` so host code must always pass an `Endian` flag to remove
/// any ambiguity about byte order.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct DataView {
    inner: emlite::Val,
}

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
impl_dyn_cast!(DataView);

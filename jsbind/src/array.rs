use crate::any::Any;
use crate::sequence::Sequence;
use crate::utils::bind;
use emlite::FromVal;

pub type TypedArray<T> = Sequence<T>;
pub type FrozenArray<T> = TypedArray<T>;
pub type ObservableArray<T> = TypedArray<T>;
pub type Uint8Array = TypedArray<u8>;
pub type Int8Array = TypedArray<i8>;
pub type Uint32Array = TypedArray<u32>;
pub type Int32Array = TypedArray<i32>;
pub type Float32Array = TypedArray<f32>;
pub type Float64Array = TypedArray<f64>;
pub type Array = Sequence<Any>;

#[derive(Clone)]
pub struct ArrayBuffer {
    inner: emlite::Val,
}

impl ArrayBuffer {
    pub fn new(byte_len: usize) -> Self {
        let ctor = emlite::Val::global("ArrayBuffer"); // window.ArrayBuffer
        let v = ctor.new(&[byte_len.into()]);
        Self::from_val(&v)
    }

    /// `buffer.byteLength`
    pub fn byte_length(&self) -> usize {
        self.inner.get("byteLength").as_::<usize>()
    }

    /// `buffer.slice(begin, end?)`
    pub fn slice(&self, begin: usize, end: Option<usize>) -> Self {
        match end {
            Some(e) => self.inner.call("slice", &[begin.into(), e.into()]),
            None => self.inner.call("slice", &[begin.into()]),
        }
        .as_::<Self>()
    }
}

bind!(ArrayBuffer);

pub enum Endian {
    Big,
    Little,
}

impl Endian {
    pub fn to_str(&self) -> &'static str {
        match *self {
            Endian::Big => "bigEndian",
            Endian::Little => "littleEndian",
        }
    }
}

macro_rules! rw {
    ($get:ident, $set:ident, $ty:ty) => {
        pub fn $get(&self, byte_offset: usize, endian: Endian) -> $ty {
            self.inner
                .call(
                    concat!("get", stringify!($get)),
                    &[byte_offset.into(), endian.to_str().into()],
                )
                .as_::<$ty>()
        }
        pub fn $set(&self, byte_offset: usize, value: $ty, endian: Endian) {
            self.inner.call(
                concat!("set", stringify!($get)),
                &[byte_offset.into(), value.into(), endian.to_str().into()],
            );
        }
    };
}

pub struct DataView {
    inner: emlite::Val,
}

impl DataView {
    pub fn new(buffer: &ArrayBuffer, offset: usize, len: Option<usize>) -> Self {
        let ctor = emlite::Val::global("DataView");
        let mut args = vec![buffer.clone().into(), offset.into()];
        if let Some(l) = len {
            args.push(l.into());
        }
        let v = ctor.new(&args);
        Self::from_val(&v)
    }

    /* ───────── helpers ───────── */

    #[inline]
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

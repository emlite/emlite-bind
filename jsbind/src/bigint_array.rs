use crate::array::ArrayBuffer;
use crate::bigint::BigInt;
use crate::utils::*;
use core::ops::{Deref, DerefMut};
use emlite::FromVal;

macro_rules! bigint_array {
    ($name:ident, $js_name:tt) => {
        #[derive(Clone, Debug, PartialEq, PartialOrd)]
        #[repr(transparent)]
        pub struct $name {
            inner: emlite::Val,
        }

        impl emlite::FromVal for $name {
            fn from_val(v: &emlite::Val) -> Self {
                Self { inner: v.clone() }
            }
            fn take_ownership(h: emlite::common::Handle) -> Self {
                Self::from_val(&emlite::Val::take_ownership(h))
            }
            fn as_handle(&self) -> emlite::common::Handle {
                self.inner.as_handle()
            }
        }

        impl From<$name> for emlite::Val {
            fn from(x: $name) -> emlite::Val {
                let handle = x.inner.as_handle();
                core::mem::forget(x);
                emlite::Val::take_ownership(handle)
            }
        }

        impl From<&$name> for emlite::Val {
            fn from(x: &$name) -> emlite::Val {
                x.inner.clone()
            }
        }

        impl Deref for $name {
            type Target = emlite::Val;

            fn deref(&self) -> &Self::Target {
                &self.inner
            }
        }

        impl DerefMut for $name {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.inner
            }
        }

        impl AsRef<emlite::Val> for $name {
            fn as_ref(&self) -> &emlite::Val {
                &self.inner
            }
        }

        impl AsMut<emlite::Val> for $name {
            fn as_mut(&mut self) -> &mut emlite::Val {
                &mut self.inner
            }
        }

        impl $name {
            pub fn new(length: usize) -> Self {
                let ctor = emlite::Val::global($js_name);
                let v = ctor.new(&[length.into()]);
                Self::from_val(&v)
            }

            pub fn newwith_buffer(
                buffer: &ArrayBuffer,
                byte_offset: usize,
                length: Option<usize>,
            ) -> Self {
                let ctor = emlite::Val::global($js_name);
                let v = match length {
                    Some(len) => ctor.new(&[buffer.into(), byte_offset.into(), len.into()]),
                    None => ctor.new(&[buffer.into(), byte_offset.into()]),
                };
                Self::from_val(&v)
            }

            pub fn length(&self) -> usize {
                self.inner.get("length").as_::<usize>()
            }

            pub fn byte_length(&self) -> usize {
                self.inner.get("byteLength").as_::<usize>()
            }

            pub fn byte_offset(&self) -> usize {
                self.inner.get("byteOffset").as_::<usize>()
            }

            pub fn buffer(&self) -> ArrayBuffer {
                self.inner.get("buffer").as_::<ArrayBuffer>()
            }

            pub fn get(&self, index: usize) -> BigInt {
                self.inner.get(index).as_::<BigInt>()
            }

            pub fn set(&self, index: usize, value: &BigInt) {
                self.inner.set(index, value);
            }

            pub fn at(&self, index: usize) -> Option<BigInt> {
                if index >= self.length() {
                    None
                } else {
                    Some(self.get(index))
                }
            }

            pub fn front(&self) -> Option<BigInt> {
                self.at(0)
            }

            pub fn back(&self) -> Option<BigInt> {
                let len = self.length();
                if len > 0 { self.at(len - 1) } else { None }
            }

            pub fn fill(&self, value: &BigInt, start: Option<usize>, end: Option<usize>) {
                match (start, end) {
                    (Some(s), Some(e)) => {
                        self.inner.call("fill", &[value.into(), s.into(), e.into()]);
                    }
                    (Some(s), None) => {
                        self.inner.call("fill", &[value.into(), s.into()]);
                    }
                    _ => {
                        self.inner.call("fill", &[value.into()]);
                    }
                }
            }

            pub fn slice(&self, start: Option<usize>, end: Option<usize>) -> Self {
                let result = match (start, end) {
                    (Some(s), Some(e)) => self.inner.call("slice", &[s.into(), e.into()]),
                    (Some(s), None) => self.inner.call("slice", &[s.into()]),
                    _ => self.inner.call("slice", &[]),
                };
                result.as_::<Self>()
            }

            pub fn subarray(&self, start: Option<usize>, end: Option<usize>) -> Self {
                let result = match (start, end) {
                    (Some(s), Some(e)) => self.inner.call("subarray", &[s.into(), e.into()]),
                    (Some(s), None) => self.inner.call("subarray", &[s.into()]),
                    _ => self.inner.call("subarray", &[]),
                };
                result.as_::<Self>()
            }

            pub fn copy_within(&self, target: usize, start: usize, end: Option<usize>) {
                match end {
                    Some(e) => self
                        .inner
                        .call("copyWithin", &[target.into(), start.into(), e.into()]),
                    None => self
                        .inner
                        .call("copyWithin", &[target.into(), start.into()]),
                };
            }
        }

        impl_dyn_cast!($name, $js_name);
    };
}

bigint_array!(BigInt64Array, "BigInt64Array");
bigint_array!(BigUint64Array, "BigUint64Array");

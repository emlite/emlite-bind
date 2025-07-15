use alloc::vec::Vec;
use core::marker::PhantomData;
use core::ops::{Deref, DerefMut};
use emlite::FromVal;

/// Parameterised wrapper around a JavaScript array object. No concrete JS type, but needed for WebIDL
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct Sequence<T> {
    inner: emlite::Val,
    phantom: PhantomData<T>,
}

impl<T> emlite::FromVal for Sequence<T> {
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

impl<T> From<Sequence<T>> for emlite::Val {
    fn from(x: Sequence<T>) -> emlite::Val {
        let handle = x.inner.as_handle();
        core::mem::forget(x);
        emlite::Val::take_ownership(handle)
    }
}

impl<T> From<&Sequence<T>> for emlite::Val {
    fn from(x: &Sequence<T>) -> emlite::Val {
        x.inner.clone()
    }
}

impl<T> Deref for Sequence<T> {
    type Target = emlite::Val;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T> DerefMut for Sequence<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl<T> AsRef<emlite::Val> for Sequence<T> {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}

impl<T> AsMut<emlite::Val> for Sequence<T> {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}

impl<T> Sequence<T> {
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

    /// Returns whether a value exists in the sequence.
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

    /// Returns a Rust Vec from a Sequence
    pub fn to_vec(&self) -> Vec<T>
    where
        T: FromVal,
    {
        self.iter().collect()
    }
}

pub struct SequenceIter<'a, T> {
    parent: &'a Sequence<T>,
    idx: usize,
    len: usize,
}

impl<T> Iterator for SequenceIter<'_, T>
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

impl<'a, T> IntoIterator for &'a Sequence<T>
where
    T: FromVal,
{
    type Item = T;
    type IntoIter = SequenceIter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        SequenceIter {
            parent: self,
            idx: 0,
            len: self.len(),
        }
    }
}

impl<T> Sequence<T>
where
    T: FromVal,
{
    /// Return an iterator over owned elements.
    pub fn iter(&self) -> SequenceIter<'_, T> {
        self.into_iter()
    }
}

impl<T: Clone> From<Vec<T>> for Sequence<T>
where
    emlite::Val: From<T>,
{
    #[inline]
    fn from(buf: Vec<T>) -> Self {
        // One copy from Wasm linear memory → JS Uint8Array.
        Self::new_from_slice(&buf)
    }
}

impl<T: Clone> From<&[T]> for Sequence<T>
where
    emlite::Val: From<T>,
{
    #[inline]
    fn from(slice: &[T]) -> Self {
        Self::new_from_slice(slice)
    }
}

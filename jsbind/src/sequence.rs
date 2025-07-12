use emlite::FromVal;
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};

#[derive(Clone, Debug)]
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

impl<T> From<Sequence<T>> for emlite::Val {
    fn from(x: Sequence<T>) -> emlite::Val {
        let handle = x.inner.as_handle();
        std::mem::forget(x);
        emlite::Val::take_ownership(handle)
    }
}

impl<T> Sequence<T> {
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
    #[inline]
    pub fn len(&self) -> usize {
        self.inner.get("length").as_::<usize>()
    }
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Push a single element (JS `Array.prototype.push`).
    pub fn push(&mut self, value: T)
    where
        T: Into<emlite::Val>,
    {
        self.inner.call("push", &[value.into()]);
    }

    /// Get a **copy** of the element at `idx`.
    pub fn get(&self, idx: usize) -> T
    where
        T: FromVal,
    {
        self.inner.get(idx as u32).as_::<T>()
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
            Some(v)
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
    /// Immutable iterator (mirrors the C++ `begin()/end()` pair).
    pub fn iter(&self) -> SequenceIter<'_, T> {
        self.into_iter()
    }
}

use emlite::FromVal;
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};

/// `Set<T>` – ECMAScript “Set” wrapper  (`new Set()`).
#[derive(Clone, Debug)]
pub struct Set<T> {
    inner: emlite::Val,
    _phantom: PhantomData<T>,
}

impl<T> emlite::FromVal for Set<T> {
    fn from_val(v: &emlite::Val) -> Self {
        Self {
            inner: v.clone(),
            _phantom: PhantomData,
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}

impl<T> From<Set<T>> for emlite::Val {
    fn from(x: Set<T>) -> emlite::Val {
        let handle = x.inner.as_handle();
        std::mem::forget(x);
        emlite::Val::take_ownership(handle)
    }
}

impl<T> Deref for Set<T> {
    type Target = emlite::Val;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T> DerefMut for Set<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl<T> Set<T> {
    /// `new Set()` — empty set.
    pub fn new() -> Self {
        emlite::Val::global("Set").new(&[]).as_::<Self>()
    }

    /// `set.size`
    pub fn size(&self) -> usize {
        self.inner.get("size").as_::<u32>() as usize
    }

    /// `set.add(value)` — returns `&mut Self` for chaining (matches JS).
    pub fn add(&mut self, value: T)
    where
        emlite::Val: From<T>,
    {
        self.inner.call("add", &[value.into()]);
    }

    /// `set.delete(value)` → `bool`
    pub fn delete(&mut self, value: T) -> bool
    where
        emlite::Val: From<T>,
    {
        self.inner.call("delete", &[value.into()]).as_::<bool>()
    }

    /// `set.clear()`
    pub fn clear(&mut self) {
        self.inner.call("clear", &[]);
    }

    /// Return a copy of the element at `idx` converted to `T`.
    pub fn get(&self, idx: usize) -> T
    where
        T: FromVal,
    {
        self.inner.get(idx).as_::<T>()
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

    #[inline]
    pub fn len(&self) -> usize {
        self.inner.get("length").as_::<usize>()
    }
}

pub struct SetIter<'a, T> {
    it: crate::Set<emlite::Val>,
    idx: usize,
    _phantom: PhantomData<&'a T>,
}

impl<'a, T> IntoIterator for &'a Set<T>
where
    T: FromVal,
{
    type Item = T;
    type IntoIter = SetIter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        let iter = self.inner.call("values", &[]);
        let vec = emlite::Val::global("Array")
            .call("from", &[iter])
            .as_::<crate::Set<emlite::Val>>();
        SetIter {
            it: vec,
            idx: 0,
            _phantom: PhantomData,
        }
    }
}

impl<'a, T> Iterator for SetIter<'a, T>
where
    T: FromVal,
{
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        if self.idx < self.it.len() {
            let v = self.it.get(self.idx).as_::<T>();
            self.idx += 1;
            Some(v)
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.it.len() - self.idx;
        (remaining, Some(remaining))
    }
}

impl<T> std::iter::FromIterator<T> for Set<T>
where
    emlite::Val: From<T>,
{
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut set = Self::new();
        for v in iter {
            set.add(v);
        }
        set
    }
}

impl<T> std::fmt::Display for Set<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s: String = self.inner.call("toString", &[]).as_();
        f.write_str(&s)
    }
}

/// `WeakSet<T>` – ECMAScript “WeakSet” wrapper  (`new WeakSet()`).
#[derive(Clone, Debug)]
pub struct WeakSet<T> {
    inner: emlite::Val,
    _phantom: PhantomData<T>,
}

impl<T> emlite::FromVal for WeakSet<T> {
    fn from_val(v: &emlite::Val) -> Self {
        Self {
            inner: v.clone(),
            _phantom: PhantomData,
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}

impl<T> From<WeakSet<T>> for emlite::Val {
    fn from(x: WeakSet<T>) -> emlite::Val {
        let handle = x.inner.as_handle();
        std::mem::forget(x);
        emlite::Val::take_ownership(handle)
    }
}

impl<T> Deref for WeakSet<T> {
    type Target = emlite::Val;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T> DerefMut for WeakSet<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl<T> WeakSet<T> {
    /// `new WeakSet()` — empty set.
    pub fn new() -> Self {
        emlite::Val::global("WeakSet").new(&[]).as_::<Self>()
    }

    /// `set.size`
    pub fn size(&self) -> usize {
        self.inner.get("size").as_::<u32>() as usize
    }

    /// `set.add(value)` — returns `&mut Self` for chaining (matches JS).
    pub fn add(&mut self, value: T)
    where
        emlite::Val: From<T>,
    {
        self.inner.call("add", &[value.into()]);
    }

    /// `set.delete(value)` → `bool`
    pub fn delete(&mut self, value: T) -> bool
    where
        emlite::Val: From<T>,
    {
        self.inner.call("delete", &[value.into()]).as_::<bool>()
    }

    /// `set.clear()`
    pub fn clear(&mut self) {
        self.inner.call("clear", &[]);
    }

    /// Return a copy of the element at `idx` converted to `T`.
    pub fn get(&self, idx: usize) -> T
    where
        T: FromVal,
    {
        self.inner.get(idx).as_::<T>()
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

    #[inline]
    pub fn len(&self) -> usize {
        self.inner.get("length").as_::<usize>()
    }
}

pub struct WeakSetIter<'a, T> {
    it: crate::WeakSet<emlite::Val>,
    idx: usize,
    _phantom: PhantomData<&'a T>,
}

impl<'a, T> IntoIterator for &'a WeakSet<T>
where
    T: FromVal,
{
    type Item = T;
    type IntoIter = WeakSetIter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        let iter = self.inner.call("values", &[]);
        let vec = emlite::Val::global("Array")
            .call("from", &[iter])
            .as_::<crate::WeakSet<emlite::Val>>();
        WeakSetIter {
            it: vec,
            idx: 0,
            _phantom: PhantomData,
        }
    }
}

impl<'a, T> Iterator for WeakSetIter<'a, T>
where
    T: FromVal,
{
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        if self.idx < self.it.len() {
            let v = self.it.get(self.idx).as_::<T>();
            self.idx += 1;
            Some(v)
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.it.len() - self.idx;
        (remaining, Some(remaining))
    }
}

impl<T> std::iter::FromIterator<T> for WeakSet<T>
where
    emlite::Val: From<T>,
{
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut set = Self::new();
        for v in iter {
            set.add(v);
        }
        set
    }
}

impl<T> std::fmt::Display for WeakSet<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s: String = self.inner.call("toString", &[]).as_();
        f.write_str(&s)
    }
}

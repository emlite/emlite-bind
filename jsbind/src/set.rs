use crate::any::Any;
use alloc::string::String;
use core::marker::PhantomData;
use core::ops::{Deref, DerefMut};
use emlite::FromVal;

/// `TypedSet<T>` – Typed wrapper around ECMAScript “Set”  (`new Set()`).
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct TypedSet<T> {
    inner: emlite::Val,
    _phantom: PhantomData<T>,
}

impl<T> emlite::FromVal for TypedSet<T> {
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

impl<T> From<TypedSet<T>> for emlite::Val {
    fn from(x: TypedSet<T>) -> emlite::Val {
        let handle = x.inner.as_handle();
        core::mem::forget(x);
        emlite::Val::take_ownership(handle)
    }
}

impl<T> Deref for TypedSet<T> {
    type Target = emlite::Val;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T> DerefMut for TypedSet<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl<T> AsRef<emlite::Val> for TypedSet<T> {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}

impl<T> AsMut<emlite::Val> for TypedSet<T> {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}

impl<T> TypedSet<T> {
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

    #[inline]
    pub fn len(&self) -> usize {
        self.inner.get("length").as_::<usize>()
    }
}

pub struct TypedSetIter<'a, T> {
    it: TypedSet<T>,
    idx: usize,
    _phantom: PhantomData<&'a T>,
}

impl<'a, T> IntoIterator for &'a TypedSet<T>
where
    T: FromVal,
{
    type Item = T;
    type IntoIter = TypedSetIter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        let iter = self.inner.call("values", &[]);
        let vec = emlite::Val::global("Array")
            .call("from", &[iter])
            .as_::<TypedSet<T>>();
        TypedSetIter {
            it: vec,
            idx: 0,
            _phantom: PhantomData,
        }
    }
}

impl<'a, T> Iterator for TypedSetIter<'a, T>
where
    T: FromVal,
{
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        if self.idx < self.it.len() {
            let v = self.it.get(self.idx);
            self.idx += 1;
            v
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.it.len() - self.idx;
        (remaining, Some(remaining))
    }
}

impl<T> core::iter::FromIterator<T> for TypedSet<T>
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

impl<T> core::fmt::Display for TypedSet<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let s: String = self.inner.call("toString", &[]).as_();
        f.write_str(&s)
    }
}

pub type Set = TypedSet<Any>;
crate::utils::impl_dyn_cast!(Set, "Set");

/// `TypedWeakSet<T>` – Typed wrapper around ECMAScript “WeakSet”  (`new WeakSet()`).
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct TypedWeakSet<T> {
    inner: emlite::Val,
    _phantom: PhantomData<T>,
}

impl<T> emlite::FromVal for TypedWeakSet<T> {
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

impl<T> From<TypedWeakSet<T>> for emlite::Val {
    fn from(x: TypedWeakSet<T>) -> emlite::Val {
        let handle = x.inner.as_handle();
        core::mem::forget(x);
        emlite::Val::take_ownership(handle)
    }
}

impl<T> Deref for TypedWeakSet<T> {
    type Target = emlite::Val;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T> DerefMut for TypedWeakSet<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl<T> TypedWeakSet<T> {
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

    #[inline]
    pub fn len(&self) -> usize {
        self.inner.get("length").as_::<usize>()
    }
}

pub struct TypedWeakSetIter<'a, T> {
    it: TypedWeakSet<T>,
    idx: usize,
    _phantom: PhantomData<&'a T>,
}

impl<'a, T> IntoIterator for &'a TypedWeakSet<T>
where
    T: FromVal,
{
    type Item = T;
    type IntoIter = TypedWeakSetIter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        let iter = self.inner.call("values", &[]);
        let vec = emlite::Val::global("Array")
            .call("from", &[iter])
            .as_::<TypedWeakSet<T>>();
        TypedWeakSetIter {
            it: vec,
            idx: 0,
            _phantom: PhantomData,
        }
    }
}

impl<'a, T> Iterator for TypedWeakSetIter<'a, T>
where
    T: FromVal,
{
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        if self.idx < self.it.len() {
            let v = self.it.get(self.idx);
            self.idx += 1;
            v
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.it.len() - self.idx;
        (remaining, Some(remaining))
    }
}

impl<T> core::iter::FromIterator<T> for TypedWeakSet<T>
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

impl<T> core::fmt::Display for TypedWeakSet<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let s: String = self.inner.call("toString", &[]).as_();
        f.write_str(&s)
    }
}

impl<T> AsRef<emlite::Val> for TypedWeakSet<T> {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}

impl<T> AsMut<emlite::Val> for TypedWeakSet<T> {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}

pub type WeakSet = TypedWeakSet<Any>;
crate::utils::impl_dyn_cast!(WeakSet, "WeakSet");

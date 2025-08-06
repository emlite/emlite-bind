use crate::any::Any;
use crate::array::Array;
use crate::record::Record;
use core::marker::PhantomData;
use core::ops::{Deref, DerefMut};
use emlite::FromVal;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct TypedMap<K, V> {
    inner: emlite::Val,
    _phantom: PhantomData<(K, V)>,
}

impl<K, V> emlite::FromVal for TypedMap<K, V> {
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

impl<K, V> From<TypedMap<K, V>> for emlite::Val {
    fn from(x: TypedMap<K, V>) -> emlite::Val {
        let handle = x.inner.as_handle();
        core::mem::forget(x);
        emlite::Val::take_ownership(handle)
    }
}

impl<K, V> From<&TypedMap<K, V>> for emlite::Val {
    fn from(x: &TypedMap<K, V>) -> emlite::Val {
        x.inner.clone()
    }
}

impl<K, V> Deref for TypedMap<K, V> {
    type Target = emlite::Val;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<K, V> DerefMut for TypedMap<K, V> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl<K, V> AsRef<emlite::Val> for TypedMap<K, V> {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}

impl<K, V> AsMut<emlite::Val> for TypedMap<K, V> {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}

pub type Map = TypedMap<Any, Any>;
crate::utils::impl_dyn_cast!(Map, "Map");

impl<K, V> Default for TypedMap<K, V> {
    fn default() -> Self {
        Self::new()
    }
}

impl<K, V> TypedMap<K, V> {
    /// `new Map(iterable?)`
    pub fn new() -> Self {
        emlite::Val::global("Map").new(&[]).as_::<Self>()
    }

    /// JavaScript `map.size`
    pub fn size(&self) -> usize {
        self.inner.get("size").as_::<u32>() as usize
    }

    pub fn set(&self, item: K, val: V)
    where
        emlite::Val: From<K>,
        emlite::Val: From<V>,
    {
        self.inner.set(item, val);
    }

    pub fn get(&self, item: K) -> Option<V>
    where
        emlite::Val: From<K>,
        emlite::Val: From<V>,
        V: FromVal,
    {
        let v = self.inner.get(item);
        if v.is_undefined() {
            None
        } else {
            Some(v.as_::<V>())
        }
    }

    /// Returns whether a value exists in the sequence.
    pub fn has(&self, item: K) -> bool
    where
        emlite::Val: From<K>,
    {
        self.inner.has(item)
    }

    /// `map.delete(key)` → bool
    pub fn delete(&mut self, key: K) -> bool
    where
        emlite::Val: From<K>,
    {
        self.inner.call("delete", &[key.into()]).as_::<bool>()
    }

    /// `map.clear()`
    pub fn clear(&mut self) {
        self.inner.call("clear", &[]);
    }
}

pub struct TypedMapIter<'a, K, V> {
    it: Array,
    idx: usize,
    _phantom: PhantomData<(&'a K, &'a V)>,
}

impl<'a, K, V> IntoIterator for &'a TypedMap<K, V>
where
    K: FromVal,
    V: FromVal,
{
    type Item = (K, V);
    type IntoIter = TypedMapIter<'a, K, V>;

    fn into_iter(self) -> Self::IntoIter {
        let iter = self.inner.call("entries", &[]).as_::<Any>(); // JS iterator
        // Turn into array via spread [...iter]; easiest cross-env.
        let vec = emlite::Val::global("Array")
            .new(&[])
            .call("from", &[iter])
            .as_::<Array>();
        TypedMapIter {
            it: vec,
            idx: 0,
            _phantom: PhantomData,
        }
    }
}

impl<K, V> Iterator for TypedMapIter<'_, K, V>
where
    K: FromVal,
    V: FromVal,
{
    type Item = (K, V);

    fn next(&mut self) -> Option<Self::Item> {
        if self.idx < self.it.len() {
            let pair = self.it.get(self.idx); // [key,val]
            self.idx += 1;
            pair.map(|p| (p.get(0).as_::<K>(), p.get(1).as_::<V>()))
        } else {
            None
        }
    }
}

impl<K, V> From<Record<K, V>> for TypedMap<K, V>
where
    K: FromVal + Into<emlite::Val>,
    V: FromVal + Into<emlite::Val>,
{
    fn from(rec: Record<K, V>) -> Self {
        let map_ctor = emlite::Val::global("Map");
        // TypedMap entries: Object.entries(obj)
        let entries = emlite::Val::global("Object").call("entries", &[rec.clone()]);
        map_ctor.new(&[entries]).as_::<Self>()
    }
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct TypedWeakMap<K, V> {
    inner: emlite::Val,
    _phantom: PhantomData<(K, V)>,
}

impl<K, V> emlite::FromVal for TypedWeakMap<K, V> {
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

impl<K, V> From<TypedWeakMap<K, V>> for emlite::Val {
    fn from(x: TypedWeakMap<K, V>) -> emlite::Val {
        let handle = x.inner.as_handle();
        core::mem::forget(x);
        emlite::Val::take_ownership(handle)
    }
}

impl<K, V> From<&TypedWeakMap<K, V>> for emlite::Val {
    fn from(x: &TypedWeakMap<K, V>) -> emlite::Val {
        x.inner.clone()
    }
}

impl<K, V> Deref for TypedWeakMap<K, V> {
    type Target = emlite::Val;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<K, V> DerefMut for TypedWeakMap<K, V> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl<K, V> AsRef<emlite::Val> for TypedWeakMap<K, V> {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}

impl<K, V> AsMut<emlite::Val> for TypedWeakMap<K, V> {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}

impl<K, V> Default for TypedWeakMap<K, V> {
    fn default() -> Self {
        Self::new()
    }
}

impl<K, V> TypedWeakMap<K, V> {
    /// `new Map(iterable?)`
    pub fn new() -> Self {
        emlite::Val::global("WeakMap").new(&[]).as_::<Self>()
    }

    /// JavaScript `map.size`
    pub fn size(&self) -> usize {
        self.inner.get("size").as_::<u32>() as usize
    }

    pub fn set(&self, item: K, val: V)
    where
        emlite::Val: From<K>,
        emlite::Val: From<V>,
    {
        self.inner.set(item, val);
    }

    pub fn get(&self, item: K) -> Option<V>
    where
        emlite::Val: From<K>,
        emlite::Val: From<V>,
        V: FromVal,
    {
        let v = self.inner.get(item);
        if v.is_undefined() {
            None
        } else {
            Some(v.as_::<V>())
        }
    }

    /// Returns whether a value exists in the sequence.
    pub fn has(&self, item: K) -> bool
    where
        emlite::Val: From<K>,
    {
        self.inner.has(item)
    }

    /// `map.delete(key)` → bool
    pub fn delete(&mut self, key: K) -> bool
    where
        emlite::Val: From<K>,
    {
        self.inner.call("delete", &[key.into()]).as_::<bool>()
    }

    /// `map.clear()`
    pub fn clear(&mut self) {
        self.inner.call("clear", &[]);
    }
}

pub struct TypedWeakMapIter<'a, K, V> {
    it: Array,
    idx: usize,
    _phantom: PhantomData<(&'a K, &'a V)>,
}

impl<'a, K, V> IntoIterator for &'a TypedWeakMap<K, V>
where
    K: FromVal,
    V: FromVal,
{
    type Item = (K, V);
    type IntoIter = TypedWeakMapIter<'a, K, V>;

    fn into_iter(self) -> Self::IntoIter {
        let iter = self.inner.call("entries", &[]).as_::<Any>(); // JS iterator
        // Turn into array via spread [...iter]; easiest cross-env.
        let vec = emlite::Val::global("Array")
            .new(&[])
            .call("from", &[iter])
            .as_::<Array>();
        TypedWeakMapIter {
            it: vec,
            idx: 0,
            _phantom: PhantomData,
        }
    }
}

impl<K, V> Iterator for TypedWeakMapIter<'_, K, V>
where
    K: FromVal,
    V: FromVal,
{
    type Item = (K, V);

    fn next(&mut self) -> Option<Self::Item> {
        if self.idx < self.it.len() {
            let pair = self.it.get(self.idx); // [key,val]
            self.idx += 1;
            pair.map(|p| (p.get(0).as_::<K>(), p.get(1).as_::<V>()))
        } else {
            None
        }
    }
}

impl<K, V> From<Record<K, V>> for TypedWeakMap<K, V>
where
    K: FromVal + Into<emlite::Val>,
    V: FromVal + Into<emlite::Val>,
{
    fn from(rec: Record<K, V>) -> Self {
        let map_ctor = emlite::Val::global("WeakMap");
        // TypedWeakMap entries: Object.entries(obj)
        let entries = emlite::Val::global("Object").call("entries", &[rec.clone()]);
        map_ctor.new(&[entries]).as_::<Self>()
    }
}

pub type WeakMap = TypedWeakMap<Any, Any>;
crate::utils::impl_dyn_cast!(WeakMap, "WeakMap");

//! jsbind :: Map<K, V>  (JavaScript Map)
//! Thin wrapper around `new Map()` that retains generic key/value types.
use core::marker::PhantomData;
use core::ops::{Deref, DerefMut};
use emlite::FromVal;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Map<K, V> {
    inner: emlite::Val,
    _phantom: PhantomData<(K, V)>,
}

impl<K, V> emlite::FromVal for Map<K, V> {
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

impl<K, V> From<Map<K, V>> for emlite::Val {
    fn from(x: Map<K, V>) -> emlite::Val {
        let handle = x.inner.as_handle();
        core::mem::forget(x);
        emlite::Val::take_ownership(handle)
    }
}

impl<K, V> Deref for Map<K, V> {
    type Target = emlite::Val;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<K, V> DerefMut for Map<K, V> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl<K, V> Map<K, V> {
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

    pub fn get(&self, item: K) -> V
    where
        emlite::Val: From<K>,
        emlite::Val: From<V>,
        V: FromVal,
    {
        self.inner.get(item).as_::<V>()
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

pub struct MapIter<'a, K, V> {
    it: crate::Sequence<crate::Any>,
    idx: usize,
    _phantom: PhantomData<(&'a K, &'a V)>,
}

impl<'a, K, V> IntoIterator for &'a Map<K, V>
where
    K: FromVal,
    V: FromVal,
{
    type Item = (K, V);
    type IntoIter = MapIter<'a, K, V>;

    fn into_iter(self) -> Self::IntoIter {
        let iter = self.inner.call("entries", &[]).as_::<crate::Any>(); // JS iterator
        // Turn into array via spread [...iter]; easiest cross-env.
        let vec = emlite::Val::global("Array")
            .new(&[])
            .call("from", &[iter])
            .as_::<crate::Sequence<crate::Any>>();
        MapIter {
            it: vec,
            idx: 0,
            _phantom: PhantomData,
        }
    }
}

impl<'a, K, V> Iterator for MapIter<'a, K, V>
where
    K: FromVal,
    V: FromVal,
{
    type Item = (K, V);

    fn next(&mut self) -> Option<Self::Item> {
        if self.idx < self.it.len() {
            let pair = self.it.get(self.idx); // [key,val]
            self.idx += 1;
            Some((pair.get(0).as_::<K>(), pair.get(1).as_::<V>()))
        } else {
            None
        }
    }
}

impl<K, V> From<crate::Record<K, V>> for Map<K, V>
where
    K: FromVal + Into<emlite::Val>,
    V: FromVal + Into<emlite::Val>,
{
    fn from(rec: crate::Record<K, V>) -> Self {
        let map_ctor = emlite::Val::global("Map");
        // Map entries: Object.entries(obj)
        let entries = emlite::Val::global("Object").call("entries", &[rec.clone().into()]);
        map_ctor.new(&[entries]).as_::<Self>()
    }
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct WeakMap<K, V> {
    inner: emlite::Val,
    _phantom: PhantomData<(K, V)>,
}

impl<K, V> emlite::FromVal for WeakMap<K, V> {
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

impl<K, V> From<WeakMap<K, V>> for emlite::Val {
    fn from(x: WeakMap<K, V>) -> emlite::Val {
        let handle = x.inner.as_handle();
        core::mem::forget(x);
        emlite::Val::take_ownership(handle)
    }
}

impl<K, V> Deref for WeakMap<K, V> {
    type Target = emlite::Val;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<K, V> DerefMut for WeakMap<K, V> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl<K, V> WeakMap<K, V> {
    /// `new WeakMap(iterable?)`
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

    pub fn get(&self, item: K) -> V
    where
        emlite::Val: From<K>,
        emlite::Val: From<V>,
        V: FromVal,
    {
        self.inner.get(item).as_::<V>()
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

pub struct WeakMapIter<'a, K, V> {
    it: crate::Sequence<crate::Any>,
    idx: usize,
    _phantom: PhantomData<(&'a K, &'a V)>,
}

impl<'a, K, V> IntoIterator for &'a WeakMap<K, V>
where
    K: FromVal,
    V: FromVal,
{
    type Item = (K, V);
    type IntoIter = WeakMapIter<'a, K, V>;

    fn into_iter(self) -> Self::IntoIter {
        let iter = self.inner.call("entries", &[]).as_::<crate::Any>(); // JS iterator
        // Turn into array via spread [...iter]; easiest cross-env.
        let vec = emlite::Val::global("Array")
            .new(&[])
            .call("from", &[iter])
            .as_::<crate::Sequence<crate::Any>>();
        WeakMapIter {
            it: vec,
            idx: 0,
            _phantom: PhantomData,
        }
    }
}

impl<'a, K, V> Iterator for WeakMapIter<'a, K, V>
where
    K: FromVal,
    V: FromVal,
{
    type Item = (K, V);

    fn next(&mut self) -> Option<Self::Item> {
        if self.idx < self.it.len() {
            let pair = self.it.get(self.idx); // [key,val]
            self.idx += 1;
            Some((pair.get(0).as_::<K>(), pair.get(1).as_::<V>()))
        } else {
            None
        }
    }
}

impl<K, V> From<crate::Record<K, V>> for WeakMap<K, V>
where
    K: FromVal + Into<emlite::Val>,
    V: FromVal + Into<emlite::Val>,
{
    fn from(rec: crate::Record<K, V>) -> Self {
        let map_ctor = emlite::Val::global("WeakMap");
        // WeakMap entries: Object.entries(obj)
        let entries = emlite::Val::global("Object").call("entries", &[rec.clone().into()]);
        map_ctor.new(&[entries]).as_::<Self>()
    }
}

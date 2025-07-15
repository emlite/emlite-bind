use core::marker::PhantomData;
use core::ops::{Deref, DerefMut};
use emlite::FromVal;

/// Web‑IDL `record<K, V>` wrapper.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct Record<K, V> {
    inner: emlite::Val,
    _phantom: PhantomData<(K, V)>,
}

impl<K, V> emlite::FromVal for Record<K, V> {
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

impl<K, V> From<Record<K, V>> for emlite::Val {
    fn from(x: Record<K, V>) -> emlite::Val {
        let handle = x.inner.as_handle();
        core::mem::forget(x);
        emlite::Val::take_ownership(handle)
    }
}

impl<K, V> Deref for Record<K, V> {
    type Target = emlite::Val;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<K, V> DerefMut for Record<K, V> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl<K, V> AsRef<emlite::Val> for Record<K, V> {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}

impl<K, V> AsMut<emlite::Val> for Record<K, V> {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}

impl<K, V> Record<K, V> {
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
}

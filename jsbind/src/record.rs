use emlite::FromVal;
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};

pub struct Record<K, V> {
    inner: emlite::Val,
    phantom1: PhantomData<K>,
    phantom2: PhantomData<V>,
}

impl<K, V> emlite::FromVal for Record<K, V> {
    fn from_val(v: &emlite::Val) -> Self {
        Self {
            inner: v.clone(),
            phantom1: PhantomData,
            phantom2: PhantomData,
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
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

impl<K, V> From<Record<K, V>> for emlite::Val {
    fn from(x: Record<K, V>) -> emlite::Val {
        let handle = x.inner.as_handle();
        std::mem::forget(x);
        emlite::Val::take_ownership(handle)
    }
}

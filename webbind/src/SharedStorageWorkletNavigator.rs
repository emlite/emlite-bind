use super::*;

#[derive(Clone, Debug)]
pub struct SharedStorageWorkletNavigator {
    inner: emlite::Val,
}
impl FromVal for SharedStorageWorkletNavigator {
    fn from_val(v: &emlite::Val) -> Self {
        SharedStorageWorkletNavigator {
            inner: emlite::Val::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for SharedStorageWorkletNavigator {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for SharedStorageWorkletNavigator {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<SharedStorageWorkletNavigator> for emlite::Val {
    fn from(s: SharedStorageWorkletNavigator) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl SharedStorageWorkletNavigator {
    pub fn locks(&self) -> LockManager {
        self.inner.get("locks").as_::<LockManager>()
    }
}

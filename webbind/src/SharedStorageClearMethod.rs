use super::*;

#[derive(Clone, Debug)]
pub struct SharedStorageClearMethod {
    inner: SharedStorageModifierMethod,
}
impl FromVal for SharedStorageClearMethod {
    fn from_val(v: &emlite::Val) -> Self {
        SharedStorageClearMethod {
            inner: SharedStorageModifierMethod::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for SharedStorageClearMethod {
    type Target = SharedStorageModifierMethod;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for SharedStorageClearMethod {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<SharedStorageClearMethod> for emlite::Val {
    fn from(s: SharedStorageClearMethod) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl SharedStorageClearMethod {
    pub fn new0() -> SharedStorageClearMethod {
        Self {
            inner: emlite::Val::global("SharedStorageClearMethod")
                .new(&[])
                .as_::<SharedStorageModifierMethod>(),
        }
    }

    pub fn new1(options: SharedStorageModifierMethodOptions) -> SharedStorageClearMethod {
        Self {
            inner: emlite::Val::global("SharedStorageClearMethod")
                .new(&[options.into()])
                .as_::<SharedStorageModifierMethod>(),
        }
    }
}

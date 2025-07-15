use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
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
impl core::ops::Deref for SharedStorageClearMethod {
    type Target = SharedStorageModifierMethod;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for SharedStorageClearMethod {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for SharedStorageClearMethod {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for SharedStorageClearMethod {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<SharedStorageClearMethod> for emlite::Val {
    fn from(s: SharedStorageClearMethod) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&SharedStorageClearMethod> for emlite::Val {
    fn from(s: &SharedStorageClearMethod) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(SharedStorageClearMethod);

impl SharedStorageClearMethod {
    pub fn new0() -> SharedStorageClearMethod {
        Self {
            inner: emlite::Val::global("SharedStorageClearMethod")
                .new(&[])
                .as_::<SharedStorageModifierMethod>(),
        }
    }

    pub fn new1(options: &SharedStorageModifierMethodOptions) -> SharedStorageClearMethod {
        Self {
            inner: emlite::Val::global("SharedStorageClearMethod")
                .new(&[options.into()])
                .as_::<SharedStorageModifierMethod>(),
        }
    }
}

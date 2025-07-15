use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct SharedStorageSetMethod {
    inner: SharedStorageModifierMethod,
}
impl FromVal for SharedStorageSetMethod {
    fn from_val(v: &emlite::Val) -> Self {
        SharedStorageSetMethod {
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
impl core::ops::Deref for SharedStorageSetMethod {
    type Target = SharedStorageModifierMethod;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for SharedStorageSetMethod {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for SharedStorageSetMethod {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for SharedStorageSetMethod {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<SharedStorageSetMethod> for emlite::Val {
    fn from(s: SharedStorageSetMethod) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&SharedStorageSetMethod> for emlite::Val {
    fn from(s: &SharedStorageSetMethod) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(SharedStorageSetMethod);

impl SharedStorageSetMethod {
    pub fn new0(key: &str, value: &str) -> SharedStorageSetMethod {
        Self {
            inner: emlite::Val::global("SharedStorageSetMethod")
                .new(&[key.into(), value.into()])
                .as_::<SharedStorageModifierMethod>(),
        }
    }

    pub fn new1(
        key: &str,
        value: &str,
        options: &SharedStorageSetMethodOptions,
    ) -> SharedStorageSetMethod {
        Self {
            inner: emlite::Val::global("SharedStorageSetMethod")
                .new(&[key.into(), value.into(), options.into()])
                .as_::<SharedStorageModifierMethod>(),
        }
    }
}

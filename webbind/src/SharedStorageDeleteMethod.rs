use super::*;

#[derive(Clone, Debug)]
pub struct SharedStorageDeleteMethod {
    inner: SharedStorageModifierMethod,
}
impl FromVal for SharedStorageDeleteMethod {
    fn from_val(v: &emlite::Val) -> Self {
        SharedStorageDeleteMethod {
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
impl std::ops::Deref for SharedStorageDeleteMethod {
    type Target = SharedStorageModifierMethod;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for SharedStorageDeleteMethod {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<SharedStorageDeleteMethod> for emlite::Val {
    fn from(s: SharedStorageDeleteMethod) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl SharedStorageDeleteMethod {
    pub fn new0(key: jsbind::DOMString) -> SharedStorageDeleteMethod {
        Self {
            inner: emlite::Val::global("SharedStorageDeleteMethod")
                .new(&[key.into()])
                .as_::<SharedStorageModifierMethod>(),
        }
    }

    pub fn new1(
        key: jsbind::DOMString,
        options: SharedStorageModifierMethodOptions,
    ) -> SharedStorageDeleteMethod {
        Self {
            inner: emlite::Val::global("SharedStorageDeleteMethod")
                .new(&[key.into(), options.into()])
                .as_::<SharedStorageModifierMethod>(),
        }
    }
}

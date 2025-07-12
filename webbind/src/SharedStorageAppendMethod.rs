use super::*;

#[derive(Clone, Debug)]
pub struct SharedStorageAppendMethod {
    inner: SharedStorageModifierMethod,
}
impl FromVal for SharedStorageAppendMethod {
    fn from_val(v: &emlite::Val) -> Self {
        SharedStorageAppendMethod {
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
impl std::ops::Deref for SharedStorageAppendMethod {
    type Target = SharedStorageModifierMethod;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for SharedStorageAppendMethod {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<SharedStorageAppendMethod> for emlite::Val {
    fn from(s: SharedStorageAppendMethod) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl SharedStorageAppendMethod {
    pub fn new0(key: jsbind::DOMString, value: jsbind::DOMString) -> SharedStorageAppendMethod {
        Self {
            inner: emlite::Val::global("SharedStorageAppendMethod")
                .new(&[key.into(), value.into()])
                .as_::<SharedStorageModifierMethod>(),
        }
    }

    pub fn new1(
        key: jsbind::DOMString,
        value: jsbind::DOMString,
        options: SharedStorageModifierMethodOptions,
    ) -> SharedStorageAppendMethod {
        Self {
            inner: emlite::Val::global("SharedStorageAppendMethod")
                .new(&[key.into(), value.into(), options.into()])
                .as_::<SharedStorageModifierMethod>(),
        }
    }
}

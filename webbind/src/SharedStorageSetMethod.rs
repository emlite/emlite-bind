use super::*;

#[derive(Clone, Debug)]
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
impl std::ops::Deref for SharedStorageSetMethod {
    type Target = SharedStorageModifierMethod;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for SharedStorageSetMethod {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<SharedStorageSetMethod> for emlite::Val {
    fn from(s: SharedStorageSetMethod) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl SharedStorageSetMethod {
    pub fn new0(key: jsbind::DOMString, value: jsbind::DOMString) -> SharedStorageSetMethod {
        Self {
            inner: emlite::Val::global("SharedStorageSetMethod")
                .new(&[key.into(), value.into()])
                .as_::<SharedStorageModifierMethod>(),
        }
    }

    pub fn new1(
        key: jsbind::DOMString,
        value: jsbind::DOMString,
        options: SharedStorageSetMethodOptions,
    ) -> SharedStorageSetMethod {
        Self {
            inner: emlite::Val::global("SharedStorageSetMethod")
                .new(&[key.into(), value.into(), options.into()])
                .as_::<SharedStorageModifierMethod>(),
        }
    }
}

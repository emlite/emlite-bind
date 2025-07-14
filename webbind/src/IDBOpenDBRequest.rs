use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct IDBOpenDBRequest {
    inner: IDBRequest,
}
impl FromVal for IDBOpenDBRequest {
    fn from_val(v: &emlite::Val) -> Self {
        IDBOpenDBRequest {
            inner: IDBRequest::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for IDBOpenDBRequest {
    type Target = IDBRequest;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for IDBOpenDBRequest {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<IDBOpenDBRequest> for emlite::Val {
    fn from(s: IDBOpenDBRequest) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl IDBOpenDBRequest {
    pub fn onblocked(&self) -> jsbind::Any {
        self.inner.get("onblocked").as_::<jsbind::Any>()
    }

    pub fn set_onblocked(&mut self, value: jsbind::Any) {
        self.inner.set("onblocked", value);
    }
}
impl IDBOpenDBRequest {
    pub fn onupgradeneeded(&self) -> jsbind::Any {
        self.inner.get("onupgradeneeded").as_::<jsbind::Any>()
    }

    pub fn set_onupgradeneeded(&mut self, value: jsbind::Any) {
        self.inner.set("onupgradeneeded", value);
    }
}

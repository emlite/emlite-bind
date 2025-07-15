use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct IDBRequest {
    inner: EventTarget,
}
impl FromVal for IDBRequest {
    fn from_val(v: &emlite::Val) -> Self {
        IDBRequest {
            inner: EventTarget::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for IDBRequest {
    type Target = EventTarget;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for IDBRequest {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for IDBRequest {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for IDBRequest {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<IDBRequest> for emlite::Val {
    fn from(s: IDBRequest) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&IDBRequest> for emlite::Val {
    fn from(s: &IDBRequest) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(IDBRequest);

impl IDBRequest {
    pub fn result(&self) -> Any {
        self.inner.get("result").as_::<Any>()
    }
}
impl IDBRequest {
    pub fn error(&self) -> DOMException {
        self.inner.get("error").as_::<DOMException>()
    }
}
impl IDBRequest {
    pub fn source(&self) -> Any {
        self.inner.get("source").as_::<Any>()
    }
}
impl IDBRequest {
    pub fn transaction(&self) -> IDBTransaction {
        self.inner.get("transaction").as_::<IDBTransaction>()
    }
}
impl IDBRequest {
    pub fn ready_state(&self) -> IDBRequestReadyState {
        self.inner.get("readyState").as_::<IDBRequestReadyState>()
    }
}
impl IDBRequest {
    pub fn onsuccess(&self) -> Any {
        self.inner.get("onsuccess").as_::<Any>()
    }

    pub fn set_onsuccess(&mut self, value: &Any) {
        self.inner.set("onsuccess", value);
    }
}
impl IDBRequest {
    pub fn onerror(&self) -> Any {
        self.inner.get("onerror").as_::<Any>()
    }

    pub fn set_onerror(&mut self, value: &Any) {
        self.inner.set("onerror", value);
    }
}

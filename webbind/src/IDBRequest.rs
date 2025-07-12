use super::*;

#[derive(Clone, Debug)]
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
impl std::ops::Deref for IDBRequest {
    type Target = EventTarget;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for IDBRequest {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<IDBRequest> for emlite::Val {
    fn from(s: IDBRequest) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl IDBRequest {
    pub fn result(&self) -> jsbind::Any {
        self.inner.get("result").as_::<jsbind::Any>()
    }
}
impl IDBRequest {
    pub fn error(&self) -> DOMException {
        self.inner.get("error").as_::<DOMException>()
    }
}
impl IDBRequest {
    pub fn source(&self) -> jsbind::Any {
        self.inner.get("source").as_::<jsbind::Any>()
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
    pub fn onsuccess(&self) -> jsbind::Any {
        self.inner.get("onsuccess").as_::<jsbind::Any>()
    }

    pub fn set_onsuccess(&mut self, value: jsbind::Any) {
        self.inner.set("onsuccess", value);
    }
}
impl IDBRequest {
    pub fn onerror(&self) -> jsbind::Any {
        self.inner.get("onerror").as_::<jsbind::Any>()
    }

    pub fn set_onerror(&mut self, value: jsbind::Any) {
        self.inner.set("onerror", value);
    }
}

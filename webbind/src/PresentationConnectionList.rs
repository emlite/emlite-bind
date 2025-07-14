use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct PresentationConnectionList {
    inner: EventTarget,
}
impl FromVal for PresentationConnectionList {
    fn from_val(v: &emlite::Val) -> Self {
        PresentationConnectionList {
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
impl core::ops::Deref for PresentationConnectionList {
    type Target = EventTarget;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for PresentationConnectionList {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<PresentationConnectionList> for emlite::Val {
    fn from(s: PresentationConnectionList) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl PresentationConnectionList {
    pub fn connections(&self) -> jsbind::FrozenArray<PresentationConnection> {
        self.inner
            .get("connections")
            .as_::<jsbind::FrozenArray<PresentationConnection>>()
    }
}
impl PresentationConnectionList {
    pub fn onconnectionavailable(&self) -> jsbind::Any {
        self.inner.get("onconnectionavailable").as_::<jsbind::Any>()
    }

    pub fn set_onconnectionavailable(&mut self, value: jsbind::Any) {
        self.inner.set("onconnectionavailable", value);
    }
}

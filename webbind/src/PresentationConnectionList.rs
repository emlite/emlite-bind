use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
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
impl AsRef<emlite::Val> for PresentationConnectionList {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for PresentationConnectionList {
    fn as_mut(&mut self) -> &mut emlite::Val {
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
impl From<&PresentationConnectionList> for emlite::Val {
    fn from(s: &PresentationConnectionList) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(PresentationConnectionList);

impl PresentationConnectionList {
    pub fn connections(&self) -> FrozenArray<PresentationConnection> {
        self.inner
            .get("connections")
            .as_::<FrozenArray<PresentationConnection>>()
    }
}
impl PresentationConnectionList {
    pub fn onconnectionavailable(&self) -> Any {
        self.inner.get("onconnectionavailable").as_::<Any>()
    }

    pub fn set_onconnectionavailable(&mut self, value: Any) {
        self.inner.set("onconnectionavailable", value);
    }
}

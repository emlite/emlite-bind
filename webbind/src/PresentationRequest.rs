use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct PresentationRequest {
    inner: EventTarget,
}
impl FromVal for PresentationRequest {
    fn from_val(v: &emlite::Val) -> Self {
        PresentationRequest {
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
impl core::ops::Deref for PresentationRequest {
    type Target = EventTarget;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for PresentationRequest {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for PresentationRequest {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for PresentationRequest {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<PresentationRequest> for emlite::Val {
    fn from(s: PresentationRequest) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(PresentationRequest);

impl PresentationRequest {
    pub fn new(urls: jsbind::Sequence<jsbind::USVString>) -> PresentationRequest {
        Self {
            inner: emlite::Val::global("PresentationRequest")
                .new(&[urls.into()])
                .as_::<EventTarget>(),
        }
    }
}
impl PresentationRequest {
    pub fn start(&self) -> jsbind::Promise {
        self.inner.call("start", &[]).as_::<jsbind::Promise>()
    }
}
impl PresentationRequest {
    pub fn reconnect(&self, presentation_id: jsbind::USVString) -> jsbind::Promise {
        self.inner
            .call("reconnect", &[presentation_id.into()])
            .as_::<jsbind::Promise>()
    }
}
impl PresentationRequest {
    pub fn get_availability(&self) -> jsbind::Promise {
        self.inner
            .call("getAvailability", &[])
            .as_::<jsbind::Promise>()
    }
}
impl PresentationRequest {
    pub fn onconnectionavailable(&self) -> jsbind::Any {
        self.inner.get("onconnectionavailable").as_::<jsbind::Any>()
    }

    pub fn set_onconnectionavailable(&mut self, value: jsbind::Any) {
        self.inner.set("onconnectionavailable", value);
    }
}

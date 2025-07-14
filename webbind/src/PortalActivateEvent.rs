use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct PortalActivateEvent {
    inner: Event,
}
impl FromVal for PortalActivateEvent {
    fn from_val(v: &emlite::Val) -> Self {
        PortalActivateEvent {
            inner: Event::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for PortalActivateEvent {
    type Target = Event;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for PortalActivateEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for PortalActivateEvent {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for PortalActivateEvent {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<PortalActivateEvent> for emlite::Val {
    fn from(s: PortalActivateEvent) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(PortalActivateEvent);

impl PortalActivateEvent {
    pub fn new0(type_: jsbind::DOMString) -> PortalActivateEvent {
        Self {
            inner: emlite::Val::global("PortalActivateEvent")
                .new(&[type_.into()])
                .as_::<Event>(),
        }
    }

    pub fn new1(type_: jsbind::DOMString, event_init_dict: jsbind::Any) -> PortalActivateEvent {
        Self {
            inner: emlite::Val::global("PortalActivateEvent")
                .new(&[type_.into(), event_init_dict.into()])
                .as_::<Event>(),
        }
    }
}
impl PortalActivateEvent {
    pub fn data(&self) -> jsbind::Any {
        self.inner.get("data").as_::<jsbind::Any>()
    }
}
impl PortalActivateEvent {
    pub fn adopt_predecessor(&self) -> HTMLPortalElement {
        self.inner
            .call("adoptPredecessor", &[])
            .as_::<HTMLPortalElement>()
    }
}

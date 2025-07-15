use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct XRReferenceSpaceEvent {
    inner: Event,
}
impl FromVal for XRReferenceSpaceEvent {
    fn from_val(v: &emlite::Val) -> Self {
        XRReferenceSpaceEvent {
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
impl core::ops::Deref for XRReferenceSpaceEvent {
    type Target = Event;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for XRReferenceSpaceEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for XRReferenceSpaceEvent {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for XRReferenceSpaceEvent {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<XRReferenceSpaceEvent> for emlite::Val {
    fn from(s: XRReferenceSpaceEvent) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(XRReferenceSpaceEvent);

impl XRReferenceSpaceEvent {
    pub fn new(type_: DOMString, event_init_dict: Any) -> XRReferenceSpaceEvent {
        Self {
            inner: emlite::Val::global("XRReferenceSpaceEvent")
                .new(&[type_.into(), event_init_dict.into()])
                .as_::<Event>(),
        }
    }
}
impl XRReferenceSpaceEvent {
    pub fn reference_space(&self) -> XRReferenceSpace {
        self.inner.get("referenceSpace").as_::<XRReferenceSpace>()
    }
}
impl XRReferenceSpaceEvent {
    pub fn transform(&self) -> XRRigidTransform {
        self.inner.get("transform").as_::<XRRigidTransform>()
    }
}

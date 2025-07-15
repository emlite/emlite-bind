use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct XRLayerEvent {
    inner: Event,
}
impl FromVal for XRLayerEvent {
    fn from_val(v: &emlite::Val) -> Self {
        XRLayerEvent {
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
impl core::ops::Deref for XRLayerEvent {
    type Target = Event;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for XRLayerEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for XRLayerEvent {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for XRLayerEvent {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<XRLayerEvent> for emlite::Val {
    fn from(s: XRLayerEvent) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(XRLayerEvent);

impl XRLayerEvent {
    pub fn new(type_: DOMString, event_init_dict: Any) -> XRLayerEvent {
        Self {
            inner: emlite::Val::global("XRLayerEvent")
                .new(&[type_.into(), event_init_dict.into()])
                .as_::<Event>(),
        }
    }
}
impl XRLayerEvent {
    pub fn layer(&self) -> XRLayer {
        self.inner.get("layer").as_::<XRLayer>()
    }
}

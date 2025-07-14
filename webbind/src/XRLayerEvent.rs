use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
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
impl From<XRLayerEvent> for emlite::Val {
    fn from(s: XRLayerEvent) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl XRLayerEvent {
    pub fn new(type_: jsbind::DOMString, event_init_dict: jsbind::Any) -> XRLayerEvent {
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

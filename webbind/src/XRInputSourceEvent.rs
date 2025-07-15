use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct XRInputSourceEvent {
    inner: Event,
}
impl FromVal for XRInputSourceEvent {
    fn from_val(v: &emlite::Val) -> Self {
        XRInputSourceEvent {
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
impl core::ops::Deref for XRInputSourceEvent {
    type Target = Event;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for XRInputSourceEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for XRInputSourceEvent {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for XRInputSourceEvent {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<XRInputSourceEvent> for emlite::Val {
    fn from(s: XRInputSourceEvent) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&XRInputSourceEvent> for emlite::Val {
    fn from(s: &XRInputSourceEvent) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(XRInputSourceEvent);

impl XRInputSourceEvent {
    pub fn new(type_: DOMString, event_init_dict: Any) -> XRInputSourceEvent {
        Self {
            inner: emlite::Val::global("XRInputSourceEvent")
                .new(&[type_.into(), event_init_dict.into()])
                .as_::<Event>(),
        }
    }
}
impl XRInputSourceEvent {
    pub fn frame(&self) -> XRFrame {
        self.inner.get("frame").as_::<XRFrame>()
    }
}
impl XRInputSourceEvent {
    pub fn input_source(&self) -> XRInputSource {
        self.inner.get("inputSource").as_::<XRInputSource>()
    }
}

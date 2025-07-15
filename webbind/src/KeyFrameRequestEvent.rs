use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct KeyFrameRequestEvent {
    inner: Event,
}
impl FromVal for KeyFrameRequestEvent {
    fn from_val(v: &emlite::Val) -> Self {
        KeyFrameRequestEvent {
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
impl core::ops::Deref for KeyFrameRequestEvent {
    type Target = Event;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for KeyFrameRequestEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for KeyFrameRequestEvent {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for KeyFrameRequestEvent {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<KeyFrameRequestEvent> for emlite::Val {
    fn from(s: KeyFrameRequestEvent) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&KeyFrameRequestEvent> for emlite::Val {
    fn from(s: &KeyFrameRequestEvent) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(KeyFrameRequestEvent);

impl KeyFrameRequestEvent {
    pub fn new0(type_: DOMString) -> KeyFrameRequestEvent {
        Self {
            inner: emlite::Val::global("KeyFrameRequestEvent")
                .new(&[type_.into()])
                .as_::<Event>(),
        }
    }

    pub fn new1(type_: DOMString, rid: DOMString) -> KeyFrameRequestEvent {
        Self {
            inner: emlite::Val::global("KeyFrameRequestEvent")
                .new(&[type_.into(), rid.into()])
                .as_::<Event>(),
        }
    }
}
impl KeyFrameRequestEvent {
    pub fn rid(&self) -> DOMString {
        self.inner.get("rid").as_::<DOMString>()
    }
}

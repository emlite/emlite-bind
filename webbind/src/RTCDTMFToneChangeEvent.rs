use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct RTCDTMFToneChangeEvent {
    inner: Event,
}
impl FromVal for RTCDTMFToneChangeEvent {
    fn from_val(v: &emlite::Val) -> Self {
        RTCDTMFToneChangeEvent {
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
impl core::ops::Deref for RTCDTMFToneChangeEvent {
    type Target = Event;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for RTCDTMFToneChangeEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for RTCDTMFToneChangeEvent {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for RTCDTMFToneChangeEvent {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<RTCDTMFToneChangeEvent> for emlite::Val {
    fn from(s: RTCDTMFToneChangeEvent) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&RTCDTMFToneChangeEvent> for emlite::Val {
    fn from(s: &RTCDTMFToneChangeEvent) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(RTCDTMFToneChangeEvent);

impl RTCDTMFToneChangeEvent {
    pub fn new0(type_: &str) -> RTCDTMFToneChangeEvent {
        Self {
            inner: emlite::Val::global("RTCDTMFToneChangeEvent")
                .new(&[type_.into()])
                .as_::<Event>(),
        }
    }

    pub fn new1(type_: &str, event_init_dict: &Any) -> RTCDTMFToneChangeEvent {
        Self {
            inner: emlite::Val::global("RTCDTMFToneChangeEvent")
                .new(&[type_.into(), event_init_dict.into()])
                .as_::<Event>(),
        }
    }
}
impl RTCDTMFToneChangeEvent {
    pub fn tone(&self) -> String {
        self.inner.get("tone").as_::<String>()
    }
}

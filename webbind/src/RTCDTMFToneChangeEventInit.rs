use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct RTCDTMFToneChangeEventInit {
    inner: Any,
}
impl FromVal for RTCDTMFToneChangeEventInit {
    fn from_val(v: &Any) -> Self {
        RTCDTMFToneChangeEventInit { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for RTCDTMFToneChangeEventInit {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for RTCDTMFToneChangeEventInit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for RTCDTMFToneChangeEventInit {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for RTCDTMFToneChangeEventInit {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<RTCDTMFToneChangeEventInit> for Any {
    fn from(s: RTCDTMFToneChangeEventInit) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&RTCDTMFToneChangeEventInit> for Any {
    fn from(s: &RTCDTMFToneChangeEventInit) -> Any {
        s.inner.clone()
    }
}

impl RTCDTMFToneChangeEventInit {
    pub fn tone(&self) -> JsString {
        self.inner.get("tone").as_::<JsString>()
    }

    pub fn set_tone(&mut self, value: &JsString) {
        self.inner.set("tone", value);
    }
}

use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct RTCRtcpParameters {
    inner: Any,
}
impl FromVal for RTCRtcpParameters {
    fn from_val(v: &Any) -> Self {
        RTCRtcpParameters { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for RTCRtcpParameters {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for RTCRtcpParameters {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for RTCRtcpParameters {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for RTCRtcpParameters {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<RTCRtcpParameters> for Any {
    fn from(s: RTCRtcpParameters) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&RTCRtcpParameters> for Any {
    fn from(s: &RTCRtcpParameters) -> Any {
        s.inner.clone()
    }
}

impl RTCRtcpParameters {
    pub fn cname(&self) -> JsString {
        self.inner.get("cname").as_::<JsString>()
    }

    pub fn set_cname(&mut self, value: &JsString) {
        self.inner.set("cname", value);
    }
}
impl RTCRtcpParameters {
    pub fn reduced_size(&self) -> bool {
        self.inner.get("reducedSize").as_::<bool>()
    }

    pub fn set_reduced_size(&mut self, value: bool) {
        self.inner.set("reducedSize", value);
    }
}

use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct RTCIdentityValidationResult {
    inner: Any,
}
impl FromVal for RTCIdentityValidationResult {
    fn from_val(v: &Any) -> Self {
        RTCIdentityValidationResult { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for RTCIdentityValidationResult {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for RTCIdentityValidationResult {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for RTCIdentityValidationResult {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for RTCIdentityValidationResult {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<RTCIdentityValidationResult> for Any {
    fn from(s: RTCIdentityValidationResult) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&RTCIdentityValidationResult> for Any {
    fn from(s: &RTCIdentityValidationResult) -> Any {
        s.inner.clone()
    }
}

impl RTCIdentityValidationResult {
    pub fn identity(&self) -> JsString {
        self.inner.get("identity").as_::<JsString>()
    }

    pub fn set_identity(&mut self, value: &JsString) {
        self.inner.set("identity", value);
    }
}
impl RTCIdentityValidationResult {
    pub fn contents(&self) -> JsString {
        self.inner.get("contents").as_::<JsString>()
    }

    pub fn set_contents(&mut self, value: &JsString) {
        self.inner.set("contents", value);
    }
}

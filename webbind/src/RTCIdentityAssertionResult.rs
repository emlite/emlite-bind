use super::*;




/// The RTCIdentityAssertionResult dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct RTCIdentityAssertionResult {
    inner: Any,
}

impl FromVal for RTCIdentityAssertionResult {
    fn from_val(v: &Any) -> Self {
        RTCIdentityAssertionResult { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for RTCIdentityAssertionResult {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for RTCIdentityAssertionResult {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for RTCIdentityAssertionResult {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for RTCIdentityAssertionResult {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<RTCIdentityAssertionResult> for Any {
    fn from(s: RTCIdentityAssertionResult) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&RTCIdentityAssertionResult> for Any {
    fn from(s: &RTCIdentityAssertionResult) -> Any {
        s.inner.clone()
    }
}

impl RTCIdentityAssertionResult {
    /// Getter of the `idp` attribute.
    pub fn idp(&self) -> RTCIdentityProviderDetails {
        self.inner.get("idp").as_::<RTCIdentityProviderDetails>()
    }

    /// Setter of the `idp` attribute.
    pub fn set_idp(&mut self, value: &RTCIdentityProviderDetails) {
        self.inner.set("idp", value);
    }
}
impl RTCIdentityAssertionResult {
    /// Getter of the `assertion` attribute.
    pub fn assertion(&self) -> JsString {
        self.inner.get("assertion").as_::<JsString>()
    }

    /// Setter of the `assertion` attribute.
    pub fn set_assertion(&mut self, value: &JsString) {
        self.inner.set("assertion", value);
    }
}

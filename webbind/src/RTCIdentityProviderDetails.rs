use super::*;




/// The RTCIdentityProviderDetails dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct RTCIdentityProviderDetails {
    inner: Any,
}

impl FromVal for RTCIdentityProviderDetails {
    fn from_val(v: &Any) -> Self {
        RTCIdentityProviderDetails { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for RTCIdentityProviderDetails {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for RTCIdentityProviderDetails {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for RTCIdentityProviderDetails {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for RTCIdentityProviderDetails {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<RTCIdentityProviderDetails> for Any {
    fn from(s: RTCIdentityProviderDetails) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&RTCIdentityProviderDetails> for Any {
    fn from(s: &RTCIdentityProviderDetails) -> Any {
        s.inner.clone()
    }
}

impl RTCIdentityProviderDetails {
    /// Getter of the `domain` attribute.
    pub fn domain(&self) -> JsString {
        self.inner.get("domain").as_::<JsString>()
    }

    /// Setter of the `domain` attribute.
    pub fn set_domain(&mut self, value: &JsString) {
        self.inner.set("domain", value);
    }
}
impl RTCIdentityProviderDetails {
    /// Getter of the `protocol` attribute.
    pub fn protocol(&self) -> JsString {
        self.inner.get("protocol").as_::<JsString>()
    }

    /// Setter of the `protocol` attribute.
    pub fn set_protocol(&mut self, value: &JsString) {
        self.inner.set("protocol", value);
    }
}

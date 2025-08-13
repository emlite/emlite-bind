use super::*;




/// The AuthenticationExtensionsPaymentOutputs dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct AuthenticationExtensionsPaymentOutputs {
    inner: Any,
}

impl FromVal for AuthenticationExtensionsPaymentOutputs {
    fn from_val(v: &Any) -> Self {
        AuthenticationExtensionsPaymentOutputs { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for AuthenticationExtensionsPaymentOutputs {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for AuthenticationExtensionsPaymentOutputs {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for AuthenticationExtensionsPaymentOutputs {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for AuthenticationExtensionsPaymentOutputs {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<AuthenticationExtensionsPaymentOutputs> for Any {
    fn from(s: AuthenticationExtensionsPaymentOutputs) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&AuthenticationExtensionsPaymentOutputs> for Any {
    fn from(s: &AuthenticationExtensionsPaymentOutputs) -> Any {
        s.inner.clone()
    }
}

impl AuthenticationExtensionsPaymentOutputs {
    /// Getter of the `browserBoundSignature` attribute.
    pub fn browser_bound_signature(&self) -> BrowserBoundSignature {
        self.inner.get("browserBoundSignature").as_::<BrowserBoundSignature>()
    }

    /// Setter of the `browserBoundSignature` attribute.
    pub fn set_browser_bound_signature(&mut self, value: &BrowserBoundSignature) {
        self.inner.set("browserBoundSignature", value);
    }
}

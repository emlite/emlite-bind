use super::*;




/// The AuthenticatorAssertionResponse class.
/// [`AuthenticatorAssertionResponse`](https://developer.mozilla.org/en-US/docs/Web/API/AuthenticatorAssertionResponse)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct AuthenticatorAssertionResponse {
    inner: AuthenticatorResponse,
}

impl FromVal for AuthenticatorAssertionResponse {
    fn from_val(v: &Any) -> Self {
        AuthenticatorAssertionResponse { inner: AuthenticatorResponse::from_val(v) }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for AuthenticatorAssertionResponse {
    type Target = AuthenticatorResponse;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for AuthenticatorAssertionResponse {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for AuthenticatorAssertionResponse {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for AuthenticatorAssertionResponse {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<AuthenticatorAssertionResponse> for Any {
    fn from(s: AuthenticatorAssertionResponse) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&AuthenticatorAssertionResponse> for Any {
    fn from(s: &AuthenticatorAssertionResponse) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(AuthenticatorAssertionResponse);


impl AuthenticatorAssertionResponse {
    /// Getter of the `authenticatorData` attribute.
    /// [`AuthenticatorAssertionResponse.authenticatorData`](https://developer.mozilla.org/en-US/docs/Web/API/AuthenticatorAssertionResponse/authenticatorData)
    pub fn authenticator_data(&self) -> ArrayBuffer {
        self.inner.get("authenticatorData").as_::<ArrayBuffer>()
    }

}
impl AuthenticatorAssertionResponse {
    /// Getter of the `signature` attribute.
    /// [`AuthenticatorAssertionResponse.signature`](https://developer.mozilla.org/en-US/docs/Web/API/AuthenticatorAssertionResponse/signature)
    pub fn signature(&self) -> ArrayBuffer {
        self.inner.get("signature").as_::<ArrayBuffer>()
    }

}
impl AuthenticatorAssertionResponse {
    /// Getter of the `userHandle` attribute.
    /// [`AuthenticatorAssertionResponse.userHandle`](https://developer.mozilla.org/en-US/docs/Web/API/AuthenticatorAssertionResponse/userHandle)
    pub fn user_handle(&self) -> ArrayBuffer {
        self.inner.get("userHandle").as_::<ArrayBuffer>()
    }

}

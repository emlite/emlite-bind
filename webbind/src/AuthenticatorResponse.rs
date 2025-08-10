use super::*;

/// The AuthenticatorResponse class.
/// [`AuthenticatorResponse`](https://developer.mozilla.org/en-US/docs/Web/API/AuthenticatorResponse)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct AuthenticatorResponse {
    inner: Any,
}

impl FromVal for AuthenticatorResponse {
    fn from_val(v: &Any) -> Self {
        AuthenticatorResponse {
            inner: Any::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for AuthenticatorResponse {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for AuthenticatorResponse {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for AuthenticatorResponse {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for AuthenticatorResponse {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<AuthenticatorResponse> for Any {
    fn from(s: AuthenticatorResponse) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&AuthenticatorResponse> for Any {
    fn from(s: &AuthenticatorResponse) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(AuthenticatorResponse);

impl AuthenticatorResponse {
    /// Getter of the `clientDataJSON` attribute.
    /// [`AuthenticatorResponse.clientDataJSON`](https://developer.mozilla.org/en-US/docs/Web/API/AuthenticatorResponse/clientDataJSON)
    pub fn client_data_json(&self) -> ArrayBuffer {
        self.inner.get("clientDataJSON").as_::<ArrayBuffer>()
    }
}

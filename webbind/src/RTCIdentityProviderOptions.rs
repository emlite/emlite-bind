use super::*;

/// The RTCIdentityProviderOptions dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct RTCIdentityProviderOptions {
    inner: Any,
}

impl FromVal for RTCIdentityProviderOptions {
    fn from_val(v: &Any) -> Self {
        RTCIdentityProviderOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for RTCIdentityProviderOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for RTCIdentityProviderOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for RTCIdentityProviderOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for RTCIdentityProviderOptions {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<RTCIdentityProviderOptions> for Any {
    fn from(s: RTCIdentityProviderOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&RTCIdentityProviderOptions> for Any {
    fn from(s: &RTCIdentityProviderOptions) -> Any {
        s.inner.clone()
    }
}

impl RTCIdentityProviderOptions {
    /// Getter of the `protocol` attribute.
    pub fn protocol(&self) -> JsString {
        self.inner.get("protocol").as_::<JsString>()
    }

    /// Setter of the `protocol` attribute.
    pub fn set_protocol(&mut self, value: &JsString) {
        self.inner.set("protocol", value);
    }
}
impl RTCIdentityProviderOptions {
    /// Getter of the `usernameHint` attribute.
    pub fn username_hint(&self) -> JsString {
        self.inner.get("usernameHint").as_::<JsString>()
    }

    /// Setter of the `usernameHint` attribute.
    pub fn set_username_hint(&mut self, value: &JsString) {
        self.inner.set("usernameHint", value);
    }
}
impl RTCIdentityProviderOptions {
    /// Getter of the `peerIdentity` attribute.
    pub fn peer_identity(&self) -> JsString {
        self.inner.get("peerIdentity").as_::<JsString>()
    }

    /// Setter of the `peerIdentity` attribute.
    pub fn set_peer_identity(&mut self, value: &JsString) {
        self.inner.set("peerIdentity", value);
    }
}

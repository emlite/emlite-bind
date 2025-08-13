use super::*;




/// The IdentityProviderConfig dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct IdentityProviderConfig {
    inner: Any,
}

impl FromVal for IdentityProviderConfig {
    fn from_val(v: &Any) -> Self {
        IdentityProviderConfig { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for IdentityProviderConfig {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for IdentityProviderConfig {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for IdentityProviderConfig {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for IdentityProviderConfig {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<IdentityProviderConfig> for Any {
    fn from(s: IdentityProviderConfig) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&IdentityProviderConfig> for Any {
    fn from(s: &IdentityProviderConfig) -> Any {
        s.inner.clone()
    }
}

impl IdentityProviderConfig {
    /// Getter of the `configURL` attribute.
    pub fn config_url(&self) -> JsString {
        self.inner.get("configURL").as_::<JsString>()
    }

    /// Setter of the `configURL` attribute.
    pub fn set_config_url(&mut self, value: &JsString) {
        self.inner.set("configURL", value);
    }
}
impl IdentityProviderConfig {
    /// Getter of the `clientId` attribute.
    pub fn client_id(&self) -> JsString {
        self.inner.get("clientId").as_::<JsString>()
    }

    /// Setter of the `clientId` attribute.
    pub fn set_client_id(&mut self, value: &JsString) {
        self.inner.set("clientId", value);
    }
}

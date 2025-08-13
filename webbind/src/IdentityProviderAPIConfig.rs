use super::*;




/// The IdentityProviderAPIConfig dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct IdentityProviderAPIConfig {
    inner: Any,
}

impl FromVal for IdentityProviderAPIConfig {
    fn from_val(v: &Any) -> Self {
        IdentityProviderAPIConfig { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for IdentityProviderAPIConfig {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for IdentityProviderAPIConfig {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for IdentityProviderAPIConfig {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for IdentityProviderAPIConfig {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<IdentityProviderAPIConfig> for Any {
    fn from(s: IdentityProviderAPIConfig) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&IdentityProviderAPIConfig> for Any {
    fn from(s: &IdentityProviderAPIConfig) -> Any {
        s.inner.clone()
    }
}

impl IdentityProviderAPIConfig {
    /// Getter of the `accounts_endpoint` attribute.
    pub fn accounts_endpoint(&self) -> JsString {
        self.inner.get("accounts_endpoint").as_::<JsString>()
    }

    /// Setter of the `accounts_endpoint` attribute.
    pub fn set_accounts_endpoint(&mut self, value: &JsString) {
        self.inner.set("accounts_endpoint", value);
    }
}
impl IdentityProviderAPIConfig {
    /// Getter of the `client_metadata_endpoint` attribute.
    pub fn client_metadata_endpoint(&self) -> JsString {
        self.inner.get("client_metadata_endpoint").as_::<JsString>()
    }

    /// Setter of the `client_metadata_endpoint` attribute.
    pub fn set_client_metadata_endpoint(&mut self, value: &JsString) {
        self.inner.set("client_metadata_endpoint", value);
    }
}
impl IdentityProviderAPIConfig {
    /// Getter of the `id_assertion_endpoint` attribute.
    pub fn id_assertion_endpoint(&self) -> JsString {
        self.inner.get("id_assertion_endpoint").as_::<JsString>()
    }

    /// Setter of the `id_assertion_endpoint` attribute.
    pub fn set_id_assertion_endpoint(&mut self, value: &JsString) {
        self.inner.set("id_assertion_endpoint", value);
    }
}
impl IdentityProviderAPIConfig {
    /// Getter of the `login_url` attribute.
    pub fn login_url(&self) -> JsString {
        self.inner.get("login_url").as_::<JsString>()
    }

    /// Setter of the `login_url` attribute.
    pub fn set_login_url(&mut self, value: &JsString) {
        self.inner.set("login_url", value);
    }
}
impl IdentityProviderAPIConfig {
    /// Getter of the `disconnect_endpoint` attribute.
    pub fn disconnect_endpoint(&self) -> JsString {
        self.inner.get("disconnect_endpoint").as_::<JsString>()
    }

    /// Setter of the `disconnect_endpoint` attribute.
    pub fn set_disconnect_endpoint(&mut self, value: &JsString) {
        self.inner.set("disconnect_endpoint", value);
    }
}
impl IdentityProviderAPIConfig {
    /// Getter of the `branding` attribute.
    pub fn branding(&self) -> IdentityProviderBranding {
        self.inner.get("branding").as_::<IdentityProviderBranding>()
    }

    /// Setter of the `branding` attribute.
    pub fn set_branding(&mut self, value: &IdentityProviderBranding) {
        self.inner.set("branding", value);
    }
}
impl IdentityProviderAPIConfig {
    /// Getter of the `supports_use_other_account` attribute.
    pub fn supports_use_other_account(&self) -> bool {
        self.inner.get("supports_use_other_account").as_::<bool>()
    }

    /// Setter of the `supports_use_other_account` attribute.
    pub fn set_supports_use_other_account(&mut self, value: bool) {
        self.inner.set("supports_use_other_account", value);
    }
}
impl IdentityProviderAPIConfig {
    /// Getter of the `account_label` attribute.
    pub fn account_label(&self) -> JsString {
        self.inner.get("account_label").as_::<JsString>()
    }

    /// Setter of the `account_label` attribute.
    pub fn set_account_label(&mut self, value: &JsString) {
        self.inner.set("account_label", value);
    }
}

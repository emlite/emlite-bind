use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct IdentityCredentialDisconnectOptions {
    inner: Any,
}
impl FromVal for IdentityCredentialDisconnectOptions {
    fn from_val(v: &Any) -> Self {
        IdentityCredentialDisconnectOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for IdentityCredentialDisconnectOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for IdentityCredentialDisconnectOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for IdentityCredentialDisconnectOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for IdentityCredentialDisconnectOptions {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<IdentityCredentialDisconnectOptions> for Any {
    fn from(s: IdentityCredentialDisconnectOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&IdentityCredentialDisconnectOptions> for Any {
    fn from(s: &IdentityCredentialDisconnectOptions) -> Any {
        s.inner.clone()
    }
}

impl IdentityCredentialDisconnectOptions {
    pub fn account_hint(&self) -> String {
        self.inner.get("accountHint").as_::<String>()
    }

    pub fn set_account_hint(&mut self, value: &str) {
        self.inner.set("accountHint", value);
    }
}
/// The IdentityCredential class.
/// [`IdentityCredential`](https://developer.mozilla.org/en-US/docs/Web/API/IdentityCredential)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct IdentityCredential {
    inner: Credential,
}
impl FromVal for IdentityCredential {
    fn from_val(v: &Any) -> Self {
        IdentityCredential {
            inner: Credential::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for IdentityCredential {
    type Target = Credential;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for IdentityCredential {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for IdentityCredential {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for IdentityCredential {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<IdentityCredential> for Any {
    fn from(s: IdentityCredential) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&IdentityCredential> for Any {
    fn from(s: &IdentityCredential) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(IdentityCredential);

impl IdentityCredential {
    /// The disconnect method.
    /// [`IdentityCredential.disconnect`](https://developer.mozilla.org/en-US/docs/Web/API/IdentityCredential/disconnect)
    pub fn disconnect(options: &IdentityCredentialDisconnectOptions) -> Promise<Undefined> {
        Any::global("IdentityCredential")
            .call("disconnect", &[options.into()])
            .as_::<Promise<Undefined>>()
    }
}
impl IdentityCredential {
    /// Getter of the `token` attribute.
    /// [`IdentityCredential.token`](https://developer.mozilla.org/en-US/docs/Web/API/IdentityCredential/token)
    pub fn token(&self) -> String {
        self.inner.get("token").as_::<String>()
    }
}
impl IdentityCredential {
    /// Getter of the `isAutoSelected` attribute.
    /// [`IdentityCredential.isAutoSelected`](https://developer.mozilla.org/en-US/docs/Web/API/IdentityCredential/isAutoSelected)
    pub fn is_auto_selected(&self) -> bool {
        self.inner.get("isAutoSelected").as_::<bool>()
    }
}
impl IdentityCredential {
    /// Getter of the `configURL` attribute.
    /// [`IdentityCredential.configURL`](https://developer.mozilla.org/en-US/docs/Web/API/IdentityCredential/configURL)
    pub fn config_url(&self) -> String {
        self.inner.get("configURL").as_::<String>()
    }
}

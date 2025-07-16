use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct CredentialRequestOptions {
    inner: Any,
}
impl FromVal for CredentialRequestOptions {
    fn from_val(v: &Any) -> Self {
        CredentialRequestOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for CredentialRequestOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for CredentialRequestOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for CredentialRequestOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for CredentialRequestOptions {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<CredentialRequestOptions> for Any {
    fn from(s: CredentialRequestOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&CredentialRequestOptions> for Any {
    fn from(s: &CredentialRequestOptions) -> Any {
        s.inner.clone()
    }
}

impl CredentialRequestOptions {
    pub fn public_key(&self) -> PublicKeyCredentialRequestOptions {
        self.inner
            .get("publicKey")
            .as_::<PublicKeyCredentialRequestOptions>()
    }

    pub fn set_public_key(&mut self, value: &PublicKeyCredentialRequestOptions) {
        self.inner.set("publicKey", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct CredentialCreationOptions {
    inner: Any,
}
impl FromVal for CredentialCreationOptions {
    fn from_val(v: &Any) -> Self {
        CredentialCreationOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for CredentialCreationOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for CredentialCreationOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for CredentialCreationOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for CredentialCreationOptions {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<CredentialCreationOptions> for Any {
    fn from(s: CredentialCreationOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&CredentialCreationOptions> for Any {
    fn from(s: &CredentialCreationOptions) -> Any {
        s.inner.clone()
    }
}

impl CredentialCreationOptions {
    pub fn public_key(&self) -> PublicKeyCredentialCreationOptions {
        self.inner
            .get("publicKey")
            .as_::<PublicKeyCredentialCreationOptions>()
    }

    pub fn set_public_key(&mut self, value: &PublicKeyCredentialCreationOptions) {
        self.inner.set("publicKey", value);
    }
}
/// The CredentialsContainer class.
/// [`CredentialsContainer`](https://developer.mozilla.org/en-US/docs/Web/API/CredentialsContainer)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct CredentialsContainer {
    inner: Any,
}
impl FromVal for CredentialsContainer {
    fn from_val(v: &Any) -> Self {
        CredentialsContainer {
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
impl core::ops::Deref for CredentialsContainer {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for CredentialsContainer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for CredentialsContainer {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for CredentialsContainer {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<CredentialsContainer> for Any {
    fn from(s: CredentialsContainer) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&CredentialsContainer> for Any {
    fn from(s: &CredentialsContainer) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(CredentialsContainer);

impl CredentialsContainer {
    /// The get method.
    /// [`CredentialsContainer.get`](https://developer.mozilla.org/en-US/docs/Web/API/CredentialsContainer/get)
    pub fn get0(&self) -> Promise {
        self.inner.call("get", &[]).as_::<Promise>()
    }
    /// The get method.
    /// [`CredentialsContainer.get`](https://developer.mozilla.org/en-US/docs/Web/API/CredentialsContainer/get)
    pub fn get1(&self, options: &CredentialRequestOptions) -> Promise {
        self.inner.call("get", &[options.into()]).as_::<Promise>()
    }
}
impl CredentialsContainer {
    /// The store method.
    /// [`CredentialsContainer.store`](https://developer.mozilla.org/en-US/docs/Web/API/CredentialsContainer/store)
    pub fn store(&self, credential: &Credential) -> Promise {
        self.inner
            .call("store", &[credential.into()])
            .as_::<Promise>()
    }
}
impl CredentialsContainer {
    /// The create method.
    /// [`CredentialsContainer.create`](https://developer.mozilla.org/en-US/docs/Web/API/CredentialsContainer/create)
    pub fn create0(&self) -> Promise {
        self.inner.call("create", &[]).as_::<Promise>()
    }
    /// The create method.
    /// [`CredentialsContainer.create`](https://developer.mozilla.org/en-US/docs/Web/API/CredentialsContainer/create)
    pub fn create1(&self, options: &CredentialCreationOptions) -> Promise {
        self.inner
            .call("create", &[options.into()])
            .as_::<Promise>()
    }
}
impl CredentialsContainer {
    /// The preventSilentAccess method.
    /// [`CredentialsContainer.preventSilentAccess`](https://developer.mozilla.org/en-US/docs/Web/API/CredentialsContainer/preventSilentAccess)
    pub fn prevent_silent_access(&self) -> Promise {
        self.inner.call("preventSilentAccess", &[]).as_::<Promise>()
    }
}

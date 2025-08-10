use super::*;

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
    pub fn get0(&self) -> Promise<Credential> {
        self.inner.call("get", &[]).as_::<Promise<Credential>>()
    }
    /// The get method.
    /// [`CredentialsContainer.get`](https://developer.mozilla.org/en-US/docs/Web/API/CredentialsContainer/get)
    pub fn get1(&self, options: &CredentialRequestOptions) -> Promise<Credential> {
        self.inner
            .call("get", &[options.into()])
            .as_::<Promise<Credential>>()
    }
}
impl CredentialsContainer {
    /// The store method.
    /// [`CredentialsContainer.store`](https://developer.mozilla.org/en-US/docs/Web/API/CredentialsContainer/store)
    pub fn store(&self, credential: &Credential) -> Promise<Undefined> {
        self.inner
            .call("store", &[credential.into()])
            .as_::<Promise<Undefined>>()
    }
}
impl CredentialsContainer {
    /// The create method.
    /// [`CredentialsContainer.create`](https://developer.mozilla.org/en-US/docs/Web/API/CredentialsContainer/create)
    pub fn create0(&self) -> Promise<Credential> {
        self.inner.call("create", &[]).as_::<Promise<Credential>>()
    }
    /// The create method.
    /// [`CredentialsContainer.create`](https://developer.mozilla.org/en-US/docs/Web/API/CredentialsContainer/create)
    pub fn create1(&self, options: &CredentialCreationOptions) -> Promise<Credential> {
        self.inner
            .call("create", &[options.into()])
            .as_::<Promise<Credential>>()
    }
}
impl CredentialsContainer {
    /// The preventSilentAccess method.
    /// [`CredentialsContainer.preventSilentAccess`](https://developer.mozilla.org/en-US/docs/Web/API/CredentialsContainer/preventSilentAccess)
    pub fn prevent_silent_access(&self) -> Promise<Undefined> {
        self.inner
            .call("preventSilentAccess", &[])
            .as_::<Promise<Undefined>>()
    }
}

use super::*;

/// The AllAcceptedCredentialsOptions dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct AllAcceptedCredentialsOptions {
    inner: Any,
}

impl FromVal for AllAcceptedCredentialsOptions {
    fn from_val(v: &Any) -> Self {
        AllAcceptedCredentialsOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for AllAcceptedCredentialsOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for AllAcceptedCredentialsOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for AllAcceptedCredentialsOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for AllAcceptedCredentialsOptions {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<AllAcceptedCredentialsOptions> for Any {
    fn from(s: AllAcceptedCredentialsOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&AllAcceptedCredentialsOptions> for Any {
    fn from(s: &AllAcceptedCredentialsOptions) -> Any {
        s.inner.clone()
    }
}

impl AllAcceptedCredentialsOptions {
    /// Getter of the `rpId` attribute.
    pub fn rp_id(&self) -> JsString {
        self.inner.get("rpId").as_::<JsString>()
    }

    /// Setter of the `rpId` attribute.
    pub fn set_rp_id(&mut self, value: &JsString) {
        self.inner.set("rpId", value);
    }
}
impl AllAcceptedCredentialsOptions {
    /// Getter of the `userId` attribute.
    pub fn user_id(&self) -> Any {
        self.inner.get("userId").as_::<Any>()
    }

    /// Setter of the `userId` attribute.
    pub fn set_user_id(&mut self, value: &Any) {
        self.inner.set("userId", value);
    }
}
impl AllAcceptedCredentialsOptions {
    /// Getter of the `allAcceptedCredentialIds` attribute.
    pub fn all_accepted_credential_ids(&self) -> TypedArray<Any> {
        self.inner
            .get("allAcceptedCredentialIds")
            .as_::<TypedArray<Any>>()
    }

    /// Setter of the `allAcceptedCredentialIds` attribute.
    pub fn set_all_accepted_credential_ids(&mut self, value: &TypedArray<Any>) {
        self.inner.set("allAcceptedCredentialIds", value);
    }
}

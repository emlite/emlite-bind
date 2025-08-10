use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct IdentityProviderClientMetadata {
    inner: Any,
}
impl FromVal for IdentityProviderClientMetadata {
    fn from_val(v: &Any) -> Self {
        IdentityProviderClientMetadata { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for IdentityProviderClientMetadata {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for IdentityProviderClientMetadata {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for IdentityProviderClientMetadata {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for IdentityProviderClientMetadata {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<IdentityProviderClientMetadata> for Any {
    fn from(s: IdentityProviderClientMetadata) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&IdentityProviderClientMetadata> for Any {
    fn from(s: &IdentityProviderClientMetadata) -> Any {
        s.inner.clone()
    }
}

impl IdentityProviderClientMetadata {
    pub fn privacy_policy_url(&self) -> JsString {
        self.inner.get("privacy_policy_url").as_::<JsString>()
    }

    pub fn set_privacy_policy_url(&mut self, value: &JsString) {
        self.inner.set("privacy_policy_url", value);
    }
}
impl IdentityProviderClientMetadata {
    pub fn terms_of_service_url(&self) -> JsString {
        self.inner.get("terms_of_service_url").as_::<JsString>()
    }

    pub fn set_terms_of_service_url(&mut self, value: &JsString) {
        self.inner.set("terms_of_service_url", value);
    }
}

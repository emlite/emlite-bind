use super::*;

/// The FederatedCredentialInit dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct FederatedCredentialInit {
    inner: Any,
}

impl FromVal for FederatedCredentialInit {
    fn from_val(v: &Any) -> Self {
        FederatedCredentialInit { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for FederatedCredentialInit {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for FederatedCredentialInit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for FederatedCredentialInit {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for FederatedCredentialInit {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<FederatedCredentialInit> for Any {
    fn from(s: FederatedCredentialInit) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&FederatedCredentialInit> for Any {
    fn from(s: &FederatedCredentialInit) -> Any {
        s.inner.clone()
    }
}

impl FederatedCredentialInit {
    /// Getter of the `name` attribute.
    pub fn name(&self) -> JsString {
        self.inner.get("name").as_::<JsString>()
    }

    /// Setter of the `name` attribute.
    pub fn set_name(&mut self, value: &JsString) {
        self.inner.set("name", value);
    }
}
impl FederatedCredentialInit {
    /// Getter of the `iconURL` attribute.
    pub fn icon_url(&self) -> JsString {
        self.inner.get("iconURL").as_::<JsString>()
    }

    /// Setter of the `iconURL` attribute.
    pub fn set_icon_url(&mut self, value: &JsString) {
        self.inner.set("iconURL", value);
    }
}
impl FederatedCredentialInit {
    /// Getter of the `origin` attribute.
    pub fn origin(&self) -> JsString {
        self.inner.get("origin").as_::<JsString>()
    }

    /// Setter of the `origin` attribute.
    pub fn set_origin(&mut self, value: &JsString) {
        self.inner.set("origin", value);
    }
}
impl FederatedCredentialInit {
    /// Getter of the `provider` attribute.
    pub fn provider(&self) -> JsString {
        self.inner.get("provider").as_::<JsString>()
    }

    /// Setter of the `provider` attribute.
    pub fn set_provider(&mut self, value: &JsString) {
        self.inner.set("provider", value);
    }
}
impl FederatedCredentialInit {
    /// Getter of the `protocol` attribute.
    pub fn protocol(&self) -> JsString {
        self.inner.get("protocol").as_::<JsString>()
    }

    /// Setter of the `protocol` attribute.
    pub fn set_protocol(&mut self, value: &JsString) {
        self.inner.set("protocol", value);
    }
}

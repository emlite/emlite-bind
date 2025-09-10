use super::*;

/// The IdentityCredentialErrorInit dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct IdentityCredentialErrorInit {
    inner: Any,
}

impl FromVal for IdentityCredentialErrorInit {
    fn from_val(v: &Any) -> Self {
        IdentityCredentialErrorInit { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for IdentityCredentialErrorInit {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for IdentityCredentialErrorInit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for IdentityCredentialErrorInit {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for IdentityCredentialErrorInit {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<IdentityCredentialErrorInit> for Any {
    fn from(s: IdentityCredentialErrorInit) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&IdentityCredentialErrorInit> for Any {
    fn from(s: &IdentityCredentialErrorInit) -> Any {
        s.inner.clone()
    }
}

impl IdentityCredentialErrorInit {
    /// Getter of the `error` attribute.
    pub fn error(&self) -> JsString {
        self.inner.get("error").as_::<JsString>()
    }

    /// Setter of the `error` attribute.
    pub fn set_error(&mut self, value: &JsString) {
        self.inner.set("error", value);
    }
}
impl IdentityCredentialErrorInit {
    /// Getter of the `url` attribute.
    pub fn url(&self) -> JsString {
        self.inner.get("url").as_::<JsString>()
    }

    /// Setter of the `url` attribute.
    pub fn set_url(&mut self, value: &JsString) {
        self.inner.set("url", value);
    }
}

use super::*;




/// The PrivateToken dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct PrivateToken {
    inner: Any,
}

impl FromVal for PrivateToken {
    fn from_val(v: &Any) -> Self {
        PrivateToken { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for PrivateToken {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for PrivateToken {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for PrivateToken {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for PrivateToken {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<PrivateToken> for Any {
    fn from(s: PrivateToken) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&PrivateToken> for Any {
    fn from(s: &PrivateToken) -> Any {
        s.inner.clone()
    }
}

impl PrivateToken {
    /// Getter of the `version` attribute.
    pub fn version(&self) -> TokenVersion {
        self.inner.get("version").as_::<TokenVersion>()
    }

    /// Setter of the `version` attribute.
    pub fn set_version(&mut self, value: &TokenVersion) {
        self.inner.set("version", value);
    }
}
impl PrivateToken {
    /// Getter of the `operation` attribute.
    pub fn operation(&self) -> OperationType {
        self.inner.get("operation").as_::<OperationType>()
    }

    /// Setter of the `operation` attribute.
    pub fn set_operation(&mut self, value: &OperationType) {
        self.inner.set("operation", value);
    }
}
impl PrivateToken {
    /// Getter of the `refreshPolicy` attribute.
    pub fn refresh_policy(&self) -> RefreshPolicy {
        self.inner.get("refreshPolicy").as_::<RefreshPolicy>()
    }

    /// Setter of the `refreshPolicy` attribute.
    pub fn set_refresh_policy(&mut self, value: &RefreshPolicy) {
        self.inner.set("refreshPolicy", value);
    }
}
impl PrivateToken {
    /// Getter of the `issuers` attribute.
    pub fn issuers(&self) -> TypedArray<JsString> {
        self.inner.get("issuers").as_::<TypedArray<JsString>>()
    }

    /// Setter of the `issuers` attribute.
    pub fn set_issuers(&mut self, value: &TypedArray<JsString>) {
        self.inner.set("issuers", value);
    }
}

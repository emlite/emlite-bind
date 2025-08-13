use super::*;




/// The IdentityResolveOptions dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct IdentityResolveOptions {
    inner: Any,
}

impl FromVal for IdentityResolveOptions {
    fn from_val(v: &Any) -> Self {
        IdentityResolveOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for IdentityResolveOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for IdentityResolveOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for IdentityResolveOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for IdentityResolveOptions {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<IdentityResolveOptions> for Any {
    fn from(s: IdentityResolveOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&IdentityResolveOptions> for Any {
    fn from(s: &IdentityResolveOptions) -> Any {
        s.inner.clone()
    }
}

impl IdentityResolveOptions {
    /// Getter of the `accountId` attribute.
    pub fn account_id(&self) -> JsString {
        self.inner.get("accountId").as_::<JsString>()
    }

    /// Setter of the `accountId` attribute.
    pub fn set_account_id(&mut self, value: &JsString) {
        self.inner.set("accountId", value);
    }
}

use super::*;




/// The CredentialPropertiesOutput dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct CredentialPropertiesOutput {
    inner: Any,
}

impl FromVal for CredentialPropertiesOutput {
    fn from_val(v: &Any) -> Self {
        CredentialPropertiesOutput { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for CredentialPropertiesOutput {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for CredentialPropertiesOutput {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for CredentialPropertiesOutput {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for CredentialPropertiesOutput {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<CredentialPropertiesOutput> for Any {
    fn from(s: CredentialPropertiesOutput) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&CredentialPropertiesOutput> for Any {
    fn from(s: &CredentialPropertiesOutput) -> Any {
        s.inner.clone()
    }
}

impl CredentialPropertiesOutput {
    /// Getter of the `rk` attribute.
    pub fn rk(&self) -> bool {
        self.inner.get("rk").as_::<bool>()
    }

    /// Setter of the `rk` attribute.
    pub fn set_rk(&mut self, value: bool) {
        self.inner.set("rk", value);
    }
}

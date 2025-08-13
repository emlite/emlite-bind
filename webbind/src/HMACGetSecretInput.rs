use super::*;




/// The HMACGetSecretInput dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct HMACGetSecretInput {
    inner: Any,
}

impl FromVal for HMACGetSecretInput {
    fn from_val(v: &Any) -> Self {
        HMACGetSecretInput { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for HMACGetSecretInput {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for HMACGetSecretInput {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for HMACGetSecretInput {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for HMACGetSecretInput {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<HMACGetSecretInput> for Any {
    fn from(s: HMACGetSecretInput) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&HMACGetSecretInput> for Any {
    fn from(s: &HMACGetSecretInput) -> Any {
        s.inner.clone()
    }
}

impl HMACGetSecretInput {
    /// Getter of the `salt1` attribute.
    pub fn salt1(&self) -> ArrayBuffer {
        self.inner.get("salt1").as_::<ArrayBuffer>()
    }

    /// Setter of the `salt1` attribute.
    pub fn set_salt1(&mut self, value: &ArrayBuffer) {
        self.inner.set("salt1", value);
    }
}
impl HMACGetSecretInput {
    /// Getter of the `salt2` attribute.
    pub fn salt2(&self) -> ArrayBuffer {
        self.inner.get("salt2").as_::<ArrayBuffer>()
    }

    /// Setter of the `salt2` attribute.
    pub fn set_salt2(&mut self, value: &ArrayBuffer) {
        self.inner.set("salt2", value);
    }
}

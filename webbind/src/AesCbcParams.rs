use super::*;




/// The AesCbcParams dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct AesCbcParams {
    inner: Any,
}

impl FromVal for AesCbcParams {
    fn from_val(v: &Any) -> Self {
        AesCbcParams { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for AesCbcParams {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for AesCbcParams {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for AesCbcParams {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for AesCbcParams {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<AesCbcParams> for Any {
    fn from(s: AesCbcParams) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&AesCbcParams> for Any {
    fn from(s: &AesCbcParams) -> Any {
        s.inner.clone()
    }
}

impl AesCbcParams {
    /// Getter of the `iv` attribute.
    pub fn iv(&self) -> Any {
        self.inner.get("iv").as_::<Any>()
    }

    /// Setter of the `iv` attribute.
    pub fn set_iv(&mut self, value: &Any) {
        self.inner.set("iv", value);
    }
}

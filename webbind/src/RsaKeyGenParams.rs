use super::*;




/// The RsaKeyGenParams dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct RsaKeyGenParams {
    inner: Any,
}

impl FromVal for RsaKeyGenParams {
    fn from_val(v: &Any) -> Self {
        RsaKeyGenParams { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for RsaKeyGenParams {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for RsaKeyGenParams {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for RsaKeyGenParams {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for RsaKeyGenParams {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<RsaKeyGenParams> for Any {
    fn from(s: RsaKeyGenParams) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&RsaKeyGenParams> for Any {
    fn from(s: &RsaKeyGenParams) -> Any {
        s.inner.clone()
    }
}

impl RsaKeyGenParams {
    /// Getter of the `modulusLength` attribute.
    pub fn modulus_length(&self) -> u32 {
        self.inner.get("modulusLength").as_::<u32>()
    }

    /// Setter of the `modulusLength` attribute.
    pub fn set_modulus_length(&mut self, value: u32) {
        self.inner.set("modulusLength", value);
    }
}
impl RsaKeyGenParams {
    /// Getter of the `publicExponent` attribute.
    pub fn public_exponent(&self) -> Any {
        self.inner.get("publicExponent").as_::<Any>()
    }

    /// Setter of the `publicExponent` attribute.
    pub fn set_public_exponent(&mut self, value: &Any) {
        self.inner.set("publicExponent", value);
    }
}

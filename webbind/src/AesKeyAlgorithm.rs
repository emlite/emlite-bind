use super::*;




/// The AesKeyAlgorithm dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct AesKeyAlgorithm {
    inner: Any,
}

impl FromVal for AesKeyAlgorithm {
    fn from_val(v: &Any) -> Self {
        AesKeyAlgorithm { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for AesKeyAlgorithm {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for AesKeyAlgorithm {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for AesKeyAlgorithm {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for AesKeyAlgorithm {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<AesKeyAlgorithm> for Any {
    fn from(s: AesKeyAlgorithm) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&AesKeyAlgorithm> for Any {
    fn from(s: &AesKeyAlgorithm) -> Any {
        s.inner.clone()
    }
}

impl AesKeyAlgorithm {
    /// Getter of the `length` attribute.
    pub fn length(&self) -> u16 {
        self.inner.get("length").as_::<u16>()
    }

    /// Setter of the `length` attribute.
    pub fn set_length(&mut self, value: u16) {
        self.inner.set("length", value);
    }
}

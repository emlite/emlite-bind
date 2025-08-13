use super::*;




/// The AesCtrParams dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct AesCtrParams {
    inner: Any,
}

impl FromVal for AesCtrParams {
    fn from_val(v: &Any) -> Self {
        AesCtrParams { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for AesCtrParams {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for AesCtrParams {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for AesCtrParams {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for AesCtrParams {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<AesCtrParams> for Any {
    fn from(s: AesCtrParams) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&AesCtrParams> for Any {
    fn from(s: &AesCtrParams) -> Any {
        s.inner.clone()
    }
}

impl AesCtrParams {
    /// Getter of the `counter` attribute.
    pub fn counter(&self) -> Any {
        self.inner.get("counter").as_::<Any>()
    }

    /// Setter of the `counter` attribute.
    pub fn set_counter(&mut self, value: &Any) {
        self.inner.set("counter", value);
    }
}
impl AesCtrParams {
    /// Getter of the `length` attribute.
    pub fn length(&self) -> u8 {
        self.inner.get("length").as_::<u8>()
    }

    /// Setter of the `length` attribute.
    pub fn set_length(&mut self, value: u8) {
        self.inner.set("length", value);
    }
}

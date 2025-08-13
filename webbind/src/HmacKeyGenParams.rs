use super::*;




/// The HmacKeyGenParams dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct HmacKeyGenParams {
    inner: Any,
}

impl FromVal for HmacKeyGenParams {
    fn from_val(v: &Any) -> Self {
        HmacKeyGenParams { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for HmacKeyGenParams {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for HmacKeyGenParams {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for HmacKeyGenParams {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for HmacKeyGenParams {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<HmacKeyGenParams> for Any {
    fn from(s: HmacKeyGenParams) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&HmacKeyGenParams> for Any {
    fn from(s: &HmacKeyGenParams) -> Any {
        s.inner.clone()
    }
}

impl HmacKeyGenParams {
    /// Getter of the `hash` attribute.
    pub fn hash(&self) -> Any {
        self.inner.get("hash").as_::<Any>()
    }

    /// Setter of the `hash` attribute.
    pub fn set_hash(&mut self, value: &Any) {
        self.inner.set("hash", value);
    }
}
impl HmacKeyGenParams {
    /// Getter of the `length` attribute.
    pub fn length(&self) -> u32 {
        self.inner.get("length").as_::<u32>()
    }

    /// Setter of the `length` attribute.
    pub fn set_length(&mut self, value: u32) {
        self.inner.set("length", value);
    }
}

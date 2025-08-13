use super::*;




/// The HmacImportParams dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct HmacImportParams {
    inner: Any,
}

impl FromVal for HmacImportParams {
    fn from_val(v: &Any) -> Self {
        HmacImportParams { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for HmacImportParams {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for HmacImportParams {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for HmacImportParams {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for HmacImportParams {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<HmacImportParams> for Any {
    fn from(s: HmacImportParams) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&HmacImportParams> for Any {
    fn from(s: &HmacImportParams) -> Any {
        s.inner.clone()
    }
}

impl HmacImportParams {
    /// Getter of the `hash` attribute.
    pub fn hash(&self) -> Any {
        self.inner.get("hash").as_::<Any>()
    }

    /// Setter of the `hash` attribute.
    pub fn set_hash(&mut self, value: &Any) {
        self.inner.set("hash", value);
    }
}
impl HmacImportParams {
    /// Getter of the `length` attribute.
    pub fn length(&self) -> u32 {
        self.inner.get("length").as_::<u32>()
    }

    /// Setter of the `length` attribute.
    pub fn set_length(&mut self, value: u32) {
        self.inner.set("length", value);
    }
}

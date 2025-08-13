use super::*;




/// The BlobPropertyBag dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct BlobPropertyBag {
    inner: Any,
}

impl FromVal for BlobPropertyBag {
    fn from_val(v: &Any) -> Self {
        BlobPropertyBag { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for BlobPropertyBag {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for BlobPropertyBag {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for BlobPropertyBag {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for BlobPropertyBag {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<BlobPropertyBag> for Any {
    fn from(s: BlobPropertyBag) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&BlobPropertyBag> for Any {
    fn from(s: &BlobPropertyBag) -> Any {
        s.inner.clone()
    }
}

impl BlobPropertyBag {
    /// Getter of the `type` attribute.
    pub fn type_(&self) -> JsString {
        self.inner.get("type").as_::<JsString>()
    }

    /// Setter of the `type` attribute.
    pub fn set_type_(&mut self, value: &JsString) {
        self.inner.set("type", value);
    }
}
impl BlobPropertyBag {
    /// Getter of the `endings` attribute.
    pub fn endings(&self) -> EndingType {
        self.inner.get("endings").as_::<EndingType>()
    }

    /// Setter of the `endings` attribute.
    pub fn set_endings(&mut self, value: &EndingType) {
        self.inner.set("endings", value);
    }
}

use super::*;




/// The GPUCopyExternalImageSourceInfo dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct GPUCopyExternalImageSourceInfo {
    inner: Any,
}

impl FromVal for GPUCopyExternalImageSourceInfo {
    fn from_val(v: &Any) -> Self {
        GPUCopyExternalImageSourceInfo { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for GPUCopyExternalImageSourceInfo {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for GPUCopyExternalImageSourceInfo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for GPUCopyExternalImageSourceInfo {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for GPUCopyExternalImageSourceInfo {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<GPUCopyExternalImageSourceInfo> for Any {
    fn from(s: GPUCopyExternalImageSourceInfo) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&GPUCopyExternalImageSourceInfo> for Any {
    fn from(s: &GPUCopyExternalImageSourceInfo) -> Any {
        s.inner.clone()
    }
}

impl GPUCopyExternalImageSourceInfo {
    /// Getter of the `source` attribute.
    pub fn source(&self) -> Any {
        self.inner.get("source").as_::<Any>()
    }

    /// Setter of the `source` attribute.
    pub fn set_source(&mut self, value: &Any) {
        self.inner.set("source", value);
    }
}
impl GPUCopyExternalImageSourceInfo {
    /// Getter of the `origin` attribute.
    pub fn origin(&self) -> Any {
        self.inner.get("origin").as_::<Any>()
    }

    /// Setter of the `origin` attribute.
    pub fn set_origin(&mut self, value: &Any) {
        self.inner.set("origin", value);
    }
}
impl GPUCopyExternalImageSourceInfo {
    /// Getter of the `flipY` attribute.
    pub fn flip_y(&self) -> bool {
        self.inner.get("flipY").as_::<bool>()
    }

    /// Setter of the `flipY` attribute.
    pub fn set_flip_y(&mut self, value: bool) {
        self.inner.set("flipY", value);
    }
}

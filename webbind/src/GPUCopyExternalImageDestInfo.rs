use super::*;




/// The GPUCopyExternalImageDestInfo dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct GPUCopyExternalImageDestInfo {
    inner: Any,
}

impl FromVal for GPUCopyExternalImageDestInfo {
    fn from_val(v: &Any) -> Self {
        GPUCopyExternalImageDestInfo { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for GPUCopyExternalImageDestInfo {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for GPUCopyExternalImageDestInfo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for GPUCopyExternalImageDestInfo {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for GPUCopyExternalImageDestInfo {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<GPUCopyExternalImageDestInfo> for Any {
    fn from(s: GPUCopyExternalImageDestInfo) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&GPUCopyExternalImageDestInfo> for Any {
    fn from(s: &GPUCopyExternalImageDestInfo) -> Any {
        s.inner.clone()
    }
}

impl GPUCopyExternalImageDestInfo {
    /// Getter of the `colorSpace` attribute.
    pub fn color_space(&self) -> PredefinedColorSpace {
        self.inner.get("colorSpace").as_::<PredefinedColorSpace>()
    }

    /// Setter of the `colorSpace` attribute.
    pub fn set_color_space(&mut self, value: &PredefinedColorSpace) {
        self.inner.set("colorSpace", value);
    }
}
impl GPUCopyExternalImageDestInfo {
    /// Getter of the `premultipliedAlpha` attribute.
    pub fn premultiplied_alpha(&self) -> bool {
        self.inner.get("premultipliedAlpha").as_::<bool>()
    }

    /// Setter of the `premultipliedAlpha` attribute.
    pub fn set_premultiplied_alpha(&mut self, value: bool) {
        self.inner.set("premultipliedAlpha", value);
    }
}

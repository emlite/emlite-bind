use super::*;




/// The GPUCanvasConfiguration dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct GPUCanvasConfiguration {
    inner: Any,
}

impl FromVal for GPUCanvasConfiguration {
    fn from_val(v: &Any) -> Self {
        GPUCanvasConfiguration { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for GPUCanvasConfiguration {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for GPUCanvasConfiguration {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for GPUCanvasConfiguration {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for GPUCanvasConfiguration {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<GPUCanvasConfiguration> for Any {
    fn from(s: GPUCanvasConfiguration) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&GPUCanvasConfiguration> for Any {
    fn from(s: &GPUCanvasConfiguration) -> Any {
        s.inner.clone()
    }
}

impl GPUCanvasConfiguration {
    /// Getter of the `device` attribute.
    pub fn device(&self) -> GPUDevice {
        self.inner.get("device").as_::<GPUDevice>()
    }

    /// Setter of the `device` attribute.
    pub fn set_device(&mut self, value: &GPUDevice) {
        self.inner.set("device", value);
    }
}
impl GPUCanvasConfiguration {
    /// Getter of the `format` attribute.
    pub fn format(&self) -> GPUTextureFormat {
        self.inner.get("format").as_::<GPUTextureFormat>()
    }

    /// Setter of the `format` attribute.
    pub fn set_format(&mut self, value: &GPUTextureFormat) {
        self.inner.set("format", value);
    }
}
impl GPUCanvasConfiguration {
    /// Getter of the `usage` attribute.
    pub fn usage(&self) -> Any {
        self.inner.get("usage").as_::<Any>()
    }

    /// Setter of the `usage` attribute.
    pub fn set_usage(&mut self, value: &Any) {
        self.inner.set("usage", value);
    }
}
impl GPUCanvasConfiguration {
    /// Getter of the `viewFormats` attribute.
    pub fn view_formats(&self) -> TypedArray<GPUTextureFormat> {
        self.inner.get("viewFormats").as_::<TypedArray<GPUTextureFormat>>()
    }

    /// Setter of the `viewFormats` attribute.
    pub fn set_view_formats(&mut self, value: &TypedArray<GPUTextureFormat>) {
        self.inner.set("viewFormats", value);
    }
}
impl GPUCanvasConfiguration {
    /// Getter of the `colorSpace` attribute.
    pub fn color_space(&self) -> PredefinedColorSpace {
        self.inner.get("colorSpace").as_::<PredefinedColorSpace>()
    }

    /// Setter of the `colorSpace` attribute.
    pub fn set_color_space(&mut self, value: &PredefinedColorSpace) {
        self.inner.set("colorSpace", value);
    }
}
impl GPUCanvasConfiguration {
    /// Getter of the `toneMapping` attribute.
    pub fn tone_mapping(&self) -> GPUCanvasToneMapping {
        self.inner.get("toneMapping").as_::<GPUCanvasToneMapping>()
    }

    /// Setter of the `toneMapping` attribute.
    pub fn set_tone_mapping(&mut self, value: &GPUCanvasToneMapping) {
        self.inner.set("toneMapping", value);
    }
}
impl GPUCanvasConfiguration {
    /// Getter of the `alphaMode` attribute.
    pub fn alpha_mode(&self) -> GPUCanvasAlphaMode {
        self.inner.get("alphaMode").as_::<GPUCanvasAlphaMode>()
    }

    /// Setter of the `alphaMode` attribute.
    pub fn set_alpha_mode(&mut self, value: &GPUCanvasAlphaMode) {
        self.inner.set("alphaMode", value);
    }
}

use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct GPURenderPassLayout {
    inner: Any,
}
impl FromVal for GPURenderPassLayout {
    fn from_val(v: &Any) -> Self {
        GPURenderPassLayout { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for GPURenderPassLayout {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for GPURenderPassLayout {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for GPURenderPassLayout {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for GPURenderPassLayout {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<GPURenderPassLayout> for Any {
    fn from(s: GPURenderPassLayout) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&GPURenderPassLayout> for Any {
    fn from(s: &GPURenderPassLayout) -> Any {
        s.inner.clone()
    }
}

impl GPURenderPassLayout {
    pub fn color_formats(&self) -> TypedArray<GPUTextureFormat> {
        self.inner
            .get("colorFormats")
            .as_::<TypedArray<GPUTextureFormat>>()
    }

    pub fn set_color_formats(&mut self, value: &TypedArray<GPUTextureFormat>) {
        self.inner.set("colorFormats", value);
    }
}
impl GPURenderPassLayout {
    pub fn depth_stencil_format(&self) -> GPUTextureFormat {
        self.inner
            .get("depthStencilFormat")
            .as_::<GPUTextureFormat>()
    }

    pub fn set_depth_stencil_format(&mut self, value: &GPUTextureFormat) {
        self.inner.set("depthStencilFormat", value);
    }
}
impl GPURenderPassLayout {
    pub fn sample_count(&self) -> Any {
        self.inner.get("sampleCount").as_::<Any>()
    }

    pub fn set_sample_count(&mut self, value: &Any) {
        self.inner.set("sampleCount", value);
    }
}

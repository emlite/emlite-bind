use super::*;

/// The GPU class.
/// [`GPU`](https://developer.mozilla.org/en-US/docs/Web/API/GPU)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct GPU {
    inner: Any,
}

impl FromVal for GPU {
    fn from_val(v: &Any) -> Self {
        GPU {
            inner: Any::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for GPU {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for GPU {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for GPU {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for GPU {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<GPU> for Any {
    fn from(s: GPU) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&GPU> for Any {
    fn from(s: &GPU) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(GPU);

impl GPU {
    /// Getter of the `wgslLanguageFeatures` attribute.
    /// [`GPU.wgslLanguageFeatures`](https://developer.mozilla.org/en-US/docs/Web/API/GPU/wgslLanguageFeatures)
    pub fn wgsl_language_features(&self) -> WGSLLanguageFeatures {
        self.inner
            .get("wgslLanguageFeatures")
            .as_::<WGSLLanguageFeatures>()
    }
}
impl GPU {
    /// The requestAdapter method.
    /// [`GPU.requestAdapter`](https://developer.mozilla.org/en-US/docs/Web/API/GPU/requestAdapter)
    pub fn request_adapter0(&self) -> Promise<GPUAdapter> {
        self.inner
            .call("requestAdapter", &[])
            .as_::<Promise<GPUAdapter>>()
    }
    /// The requestAdapter method.
    /// [`GPU.requestAdapter`](https://developer.mozilla.org/en-US/docs/Web/API/GPU/requestAdapter)
    pub fn request_adapter1(&self, options: &GPURequestAdapterOptions) -> Promise<GPUAdapter> {
        self.inner
            .call("requestAdapter", &[options.into()])
            .as_::<Promise<GPUAdapter>>()
    }
}
impl GPU {
    /// The getPreferredCanvasFormat method.
    /// [`GPU.getPreferredCanvasFormat`](https://developer.mozilla.org/en-US/docs/Web/API/GPU/getPreferredCanvasFormat)
    pub fn get_preferred_canvas_format(&self) -> GPUTextureFormat {
        self.inner
            .call("getPreferredCanvasFormat", &[])
            .as_::<GPUTextureFormat>()
    }
}

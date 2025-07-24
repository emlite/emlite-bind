use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct GPURequestAdapterOptions {
    inner: Any,
}
impl FromVal for GPURequestAdapterOptions {
    fn from_val(v: &Any) -> Self {
        GPURequestAdapterOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for GPURequestAdapterOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for GPURequestAdapterOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for GPURequestAdapterOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for GPURequestAdapterOptions {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<GPURequestAdapterOptions> for Any {
    fn from(s: GPURequestAdapterOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&GPURequestAdapterOptions> for Any {
    fn from(s: &GPURequestAdapterOptions) -> Any {
        s.inner.clone()
    }
}

impl GPURequestAdapterOptions {
    pub fn feature_level(&self) -> DOMString {
        self.inner.get("featureLevel").as_::<DOMString>()
    }

    pub fn set_feature_level(&mut self, value: &DOMString) {
        self.inner.set("featureLevel", value);
    }
}
impl GPURequestAdapterOptions {
    pub fn power_preference(&self) -> GPUPowerPreference {
        self.inner
            .get("powerPreference")
            .as_::<GPUPowerPreference>()
    }

    pub fn set_power_preference(&mut self, value: &GPUPowerPreference) {
        self.inner.set("powerPreference", value);
    }
}
impl GPURequestAdapterOptions {
    pub fn force_fallback_adapter(&self) -> bool {
        self.inner.get("forceFallbackAdapter").as_::<bool>()
    }

    pub fn set_force_fallback_adapter(&mut self, value: bool) {
        self.inner.set("forceFallbackAdapter", value);
    }
}
impl GPURequestAdapterOptions {
    pub fn xr_compatible(&self) -> bool {
        self.inner.get("xrCompatible").as_::<bool>()
    }

    pub fn set_xr_compatible(&mut self, value: bool) {
        self.inner.set("xrCompatible", value);
    }
}
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
impl GPU {
    /// Getter of the `wgslLanguageFeatures` attribute.
    /// [`GPU.wgslLanguageFeatures`](https://developer.mozilla.org/en-US/docs/Web/API/GPU/wgslLanguageFeatures)
    pub fn wgsl_language_features(&self) -> WGSLLanguageFeatures {
        self.inner
            .get("wgslLanguageFeatures")
            .as_::<WGSLLanguageFeatures>()
    }
}

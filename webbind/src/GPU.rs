use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct GPURequestAdapterOptions {
    inner: emlite::Val,
}
impl FromVal for GPURequestAdapterOptions {
    fn from_val(v: &emlite::Val) -> Self {
        GPURequestAdapterOptions { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for GPURequestAdapterOptions {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for GPURequestAdapterOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for GPURequestAdapterOptions {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for GPURequestAdapterOptions {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<GPURequestAdapterOptions> for emlite::Val {
    fn from(s: GPURequestAdapterOptions) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl GPURequestAdapterOptions {
    pub fn feature_level(&self) -> jsbind::DOMString {
        self.inner.get("featureLevel").as_::<jsbind::DOMString>()
    }

    pub fn set_feature_level(&mut self, value: jsbind::DOMString) {
        self.inner.set("featureLevel", value);
    }
}
impl GPURequestAdapterOptions {
    pub fn power_preference(&self) -> GPUPowerPreference {
        self.inner
            .get("powerPreference")
            .as_::<GPUPowerPreference>()
    }

    pub fn set_power_preference(&mut self, value: GPUPowerPreference) {
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
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct GPU {
    inner: emlite::Val,
}
impl FromVal for GPU {
    fn from_val(v: &emlite::Val) -> Self {
        GPU {
            inner: emlite::Val::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for GPU {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for GPU {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for GPU {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for GPU {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<GPU> for emlite::Val {
    fn from(s: GPU) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(GPU);

impl GPU {
    pub fn request_adapter0(&self) -> jsbind::Promise {
        self.inner
            .call("requestAdapter", &[])
            .as_::<jsbind::Promise>()
    }

    pub fn request_adapter1(&self, options: GPURequestAdapterOptions) -> jsbind::Promise {
        self.inner
            .call("requestAdapter", &[options.into()])
            .as_::<jsbind::Promise>()
    }
}
impl GPU {
    pub fn get_preferred_canvas_format(&self) -> GPUTextureFormat {
        self.inner
            .call("getPreferredCanvasFormat", &[])
            .as_::<GPUTextureFormat>()
    }
}
impl GPU {
    pub fn wgsl_language_features(&self) -> WGSLLanguageFeatures {
        self.inner
            .get("wgslLanguageFeatures")
            .as_::<WGSLLanguageFeatures>()
    }
}

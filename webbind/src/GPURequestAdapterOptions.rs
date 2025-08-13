use super::*;




/// The GPURequestAdapterOptions dictionary.
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
    /// Getter of the `featureLevel` attribute.
    pub fn feature_level(&self) -> JsString {
        self.inner.get("featureLevel").as_::<JsString>()
    }

    /// Setter of the `featureLevel` attribute.
    pub fn set_feature_level(&mut self, value: &JsString) {
        self.inner.set("featureLevel", value);
    }
}
impl GPURequestAdapterOptions {
    /// Getter of the `powerPreference` attribute.
    pub fn power_preference(&self) -> GPUPowerPreference {
        self.inner.get("powerPreference").as_::<GPUPowerPreference>()
    }

    /// Setter of the `powerPreference` attribute.
    pub fn set_power_preference(&mut self, value: &GPUPowerPreference) {
        self.inner.set("powerPreference", value);
    }
}
impl GPURequestAdapterOptions {
    /// Getter of the `forceFallbackAdapter` attribute.
    pub fn force_fallback_adapter(&self) -> bool {
        self.inner.get("forceFallbackAdapter").as_::<bool>()
    }

    /// Setter of the `forceFallbackAdapter` attribute.
    pub fn set_force_fallback_adapter(&mut self, value: bool) {
        self.inner.set("forceFallbackAdapter", value);
    }
}
impl GPURequestAdapterOptions {
    /// Getter of the `xrCompatible` attribute.
    pub fn xr_compatible(&self) -> bool {
        self.inner.get("xrCompatible").as_::<bool>()
    }

    /// Setter of the `xrCompatible` attribute.
    pub fn set_xr_compatible(&mut self, value: bool) {
        self.inner.set("xrCompatible", value);
    }
}

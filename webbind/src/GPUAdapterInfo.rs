use super::*;

/// The GPUAdapterInfo class.
/// [`GPUAdapterInfo`](https://developer.mozilla.org/en-US/docs/Web/API/GPUAdapterInfo)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct GPUAdapterInfo {
    inner: Any,
}

impl FromVal for GPUAdapterInfo {
    fn from_val(v: &Any) -> Self {
        GPUAdapterInfo {
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

impl core::ops::Deref for GPUAdapterInfo {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for GPUAdapterInfo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for GPUAdapterInfo {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for GPUAdapterInfo {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<GPUAdapterInfo> for Any {
    fn from(s: GPUAdapterInfo) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&GPUAdapterInfo> for Any {
    fn from(s: &GPUAdapterInfo) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(GPUAdapterInfo);

impl GPUAdapterInfo {
    /// Getter of the `vendor` attribute.
    /// [`GPUAdapterInfo.vendor`](https://developer.mozilla.org/en-US/docs/Web/API/GPUAdapterInfo/vendor)
    pub fn vendor(&self) -> JsString {
        self.inner.get("vendor").as_::<JsString>()
    }
}
impl GPUAdapterInfo {
    /// Getter of the `architecture` attribute.
    /// [`GPUAdapterInfo.architecture`](https://developer.mozilla.org/en-US/docs/Web/API/GPUAdapterInfo/architecture)
    pub fn architecture(&self) -> JsString {
        self.inner.get("architecture").as_::<JsString>()
    }
}
impl GPUAdapterInfo {
    /// Getter of the `device` attribute.
    /// [`GPUAdapterInfo.device`](https://developer.mozilla.org/en-US/docs/Web/API/GPUAdapterInfo/device)
    pub fn device(&self) -> JsString {
        self.inner.get("device").as_::<JsString>()
    }
}
impl GPUAdapterInfo {
    /// Getter of the `description` attribute.
    /// [`GPUAdapterInfo.description`](https://developer.mozilla.org/en-US/docs/Web/API/GPUAdapterInfo/description)
    pub fn description(&self) -> JsString {
        self.inner.get("description").as_::<JsString>()
    }
}
impl GPUAdapterInfo {
    /// Getter of the `subgroupMinSize` attribute.
    /// [`GPUAdapterInfo.subgroupMinSize`](https://developer.mozilla.org/en-US/docs/Web/API/GPUAdapterInfo/subgroupMinSize)
    pub fn subgroup_min_size(&self) -> u32 {
        self.inner.get("subgroupMinSize").as_::<u32>()
    }
}
impl GPUAdapterInfo {
    /// Getter of the `subgroupMaxSize` attribute.
    /// [`GPUAdapterInfo.subgroupMaxSize`](https://developer.mozilla.org/en-US/docs/Web/API/GPUAdapterInfo/subgroupMaxSize)
    pub fn subgroup_max_size(&self) -> u32 {
        self.inner.get("subgroupMaxSize").as_::<u32>()
    }
}
impl GPUAdapterInfo {
    /// Getter of the `isFallbackAdapter` attribute.
    /// [`GPUAdapterInfo.isFallbackAdapter`](https://developer.mozilla.org/en-US/docs/Web/API/GPUAdapterInfo/isFallbackAdapter)
    pub fn is_fallback_adapter(&self) -> bool {
        self.inner.get("isFallbackAdapter").as_::<bool>()
    }
}

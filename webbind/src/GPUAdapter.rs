use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct GPUDeviceDescriptor {
    inner: Any,
}
impl FromVal for GPUDeviceDescriptor {
    fn from_val(v: &Any) -> Self {
        GPUDeviceDescriptor { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for GPUDeviceDescriptor {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for GPUDeviceDescriptor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for GPUDeviceDescriptor {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for GPUDeviceDescriptor {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<GPUDeviceDescriptor> for Any {
    fn from(s: GPUDeviceDescriptor) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&GPUDeviceDescriptor> for Any {
    fn from(s: &GPUDeviceDescriptor) -> Any {
        s.inner.clone()
    }
}

impl GPUDeviceDescriptor {
    pub fn required_features(&self) -> TypedArray<GPUFeatureName> {
        self.inner
            .get("requiredFeatures")
            .as_::<TypedArray<GPUFeatureName>>()
    }

    pub fn set_required_features(&mut self, value: &TypedArray<GPUFeatureName>) {
        self.inner.set("requiredFeatures", value);
    }
}
impl GPUDeviceDescriptor {
    pub fn required_limits(&self) -> Record<JsString, Any> {
        self.inner
            .get("requiredLimits")
            .as_::<Record<JsString, Any>>()
    }

    pub fn set_required_limits(&mut self, value: &Record<JsString, Any>) {
        self.inner.set("requiredLimits", value);
    }
}
impl GPUDeviceDescriptor {
    pub fn default_queue(&self) -> Any {
        self.inner.get("defaultQueue").as_::<Any>()
    }

    pub fn set_default_queue(&mut self, value: &Any) {
        self.inner.set("defaultQueue", value);
    }
}
/// The GPUAdapter class.
/// [`GPUAdapter`](https://developer.mozilla.org/en-US/docs/Web/API/GPUAdapter)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct GPUAdapter {
    inner: Any,
}
impl FromVal for GPUAdapter {
    fn from_val(v: &Any) -> Self {
        GPUAdapter {
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
impl core::ops::Deref for GPUAdapter {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for GPUAdapter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for GPUAdapter {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for GPUAdapter {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<GPUAdapter> for Any {
    fn from(s: GPUAdapter) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&GPUAdapter> for Any {
    fn from(s: &GPUAdapter) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(GPUAdapter);

impl GPUAdapter {
    /// Getter of the `features` attribute.
    /// [`GPUAdapter.features`](https://developer.mozilla.org/en-US/docs/Web/API/GPUAdapter/features)
    pub fn features(&self) -> GPUSupportedFeatures {
        self.inner.get("features").as_::<GPUSupportedFeatures>()
    }
}
impl GPUAdapter {
    /// Getter of the `limits` attribute.
    /// [`GPUAdapter.limits`](https://developer.mozilla.org/en-US/docs/Web/API/GPUAdapter/limits)
    pub fn limits(&self) -> GPUSupportedLimits {
        self.inner.get("limits").as_::<GPUSupportedLimits>()
    }
}
impl GPUAdapter {
    /// Getter of the `info` attribute.
    /// [`GPUAdapter.info`](https://developer.mozilla.org/en-US/docs/Web/API/GPUAdapter/info)
    pub fn info(&self) -> GPUAdapterInfo {
        self.inner.get("info").as_::<GPUAdapterInfo>()
    }
}
impl GPUAdapter {
    /// The requestDevice method.
    /// [`GPUAdapter.requestDevice`](https://developer.mozilla.org/en-US/docs/Web/API/GPUAdapter/requestDevice)
    pub fn request_device0(&self) -> Promise<GPUDevice> {
        self.inner
            .call("requestDevice", &[])
            .as_::<Promise<GPUDevice>>()
    }
    /// The requestDevice method.
    /// [`GPUAdapter.requestDevice`](https://developer.mozilla.org/en-US/docs/Web/API/GPUAdapter/requestDevice)
    pub fn request_device1(&self, descriptor: &GPUDeviceDescriptor) -> Promise<GPUDevice> {
        self.inner
            .call("requestDevice", &[descriptor.into()])
            .as_::<Promise<GPUDevice>>()
    }
}

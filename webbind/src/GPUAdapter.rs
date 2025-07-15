use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct GPUDeviceDescriptor {
    inner: emlite::Val,
}
impl FromVal for GPUDeviceDescriptor {
    fn from_val(v: &emlite::Val) -> Self {
        GPUDeviceDescriptor { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for GPUDeviceDescriptor {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for GPUDeviceDescriptor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for GPUDeviceDescriptor {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for GPUDeviceDescriptor {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<GPUDeviceDescriptor> for emlite::Val {
    fn from(s: GPUDeviceDescriptor) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&GPUDeviceDescriptor> for emlite::Val {
    fn from(s: &GPUDeviceDescriptor) -> emlite::Val {
        s.inner.clone()
    }
}

impl GPUDeviceDescriptor {
    pub fn required_features(&self) -> Sequence<GPUFeatureName> {
        self.inner
            .get("requiredFeatures")
            .as_::<Sequence<GPUFeatureName>>()
    }

    pub fn set_required_features(&mut self, value: &Sequence<GPUFeatureName>) {
        self.inner.set("requiredFeatures", value);
    }
}
impl GPUDeviceDescriptor {
    pub fn required_limits(&self) -> Record<String, Any> {
        self.inner
            .get("requiredLimits")
            .as_::<Record<String, Any>>()
    }

    pub fn set_required_limits(&mut self, value: &Record<String, Any>) {
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
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct GPUAdapter {
    inner: emlite::Val,
}
impl FromVal for GPUAdapter {
    fn from_val(v: &emlite::Val) -> Self {
        GPUAdapter {
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
impl core::ops::Deref for GPUAdapter {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for GPUAdapter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for GPUAdapter {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for GPUAdapter {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<GPUAdapter> for emlite::Val {
    fn from(s: GPUAdapter) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&GPUAdapter> for emlite::Val {
    fn from(s: &GPUAdapter) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(GPUAdapter);

impl GPUAdapter {
    pub fn features(&self) -> GPUSupportedFeatures {
        self.inner.get("features").as_::<GPUSupportedFeatures>()
    }
}
impl GPUAdapter {
    pub fn limits(&self) -> GPUSupportedLimits {
        self.inner.get("limits").as_::<GPUSupportedLimits>()
    }
}
impl GPUAdapter {
    pub fn info(&self) -> GPUAdapterInfo {
        self.inner.get("info").as_::<GPUAdapterInfo>()
    }
}
impl GPUAdapter {
    pub fn request_device0(&self) -> Promise {
        self.inner.call("requestDevice", &[]).as_::<Promise>()
    }

    pub fn request_device1(&self, descriptor: &GPUDeviceDescriptor) -> Promise {
        self.inner
            .call("requestDevice", &[descriptor.into()])
            .as_::<Promise>()
    }
}

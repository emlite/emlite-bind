use super::*;

#[derive(Clone, Debug)]
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
impl std::ops::Deref for GPUDeviceDescriptor {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for GPUDeviceDescriptor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<GPUDeviceDescriptor> for emlite::Val {
    fn from(s: GPUDeviceDescriptor) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl GPUDeviceDescriptor {
    pub fn required_features(&self) -> jsbind::Sequence<GPUFeatureName> {
        self.inner
            .get("requiredFeatures")
            .as_::<jsbind::Sequence<GPUFeatureName>>()
    }

    pub fn set_required_features(&mut self, value: jsbind::Sequence<GPUFeatureName>) {
        self.inner.set("requiredFeatures", value);
    }
}
impl GPUDeviceDescriptor {
    pub fn required_limits(&self) -> jsbind::Record<jsbind::DOMString, jsbind::Any> {
        self.inner
            .get("requiredLimits")
            .as_::<jsbind::Record<jsbind::DOMString, jsbind::Any>>()
    }

    pub fn set_required_limits(&mut self, value: jsbind::Record<jsbind::DOMString, jsbind::Any>) {
        self.inner.set("requiredLimits", value);
    }
}
impl GPUDeviceDescriptor {
    pub fn default_queue(&self) -> jsbind::Any {
        self.inner.get("defaultQueue").as_::<jsbind::Any>()
    }

    pub fn set_default_queue(&mut self, value: jsbind::Any) {
        self.inner.set("defaultQueue", value);
    }
}
#[derive(Clone, Debug)]
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
impl std::ops::Deref for GPUAdapter {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for GPUAdapter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<GPUAdapter> for emlite::Val {
    fn from(s: GPUAdapter) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

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
    pub fn request_device0(&self) -> jsbind::Promise {
        self.inner
            .call("requestDevice", &[])
            .as_::<jsbind::Promise>()
    }

    pub fn request_device1(&self, descriptor: GPUDeviceDescriptor) -> jsbind::Promise {
        self.inner
            .call("requestDevice", &[descriptor.into()])
            .as_::<jsbind::Promise>()
    }
}

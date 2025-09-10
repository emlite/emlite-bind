use super::*;

/// The GPUDeviceDescriptor dictionary.
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
    /// Getter of the `requiredFeatures` attribute.
    pub fn required_features(&self) -> TypedArray<GPUFeatureName> {
        self.inner
            .get("requiredFeatures")
            .as_::<TypedArray<GPUFeatureName>>()
    }

    /// Setter of the `requiredFeatures` attribute.
    pub fn set_required_features(&mut self, value: &TypedArray<GPUFeatureName>) {
        self.inner.set("requiredFeatures", value);
    }
}
impl GPUDeviceDescriptor {
    /// Getter of the `requiredLimits` attribute.
    pub fn required_limits(&self) -> Record<JsString, Any> {
        self.inner
            .get("requiredLimits")
            .as_::<Record<JsString, Any>>()
    }

    /// Setter of the `requiredLimits` attribute.
    pub fn set_required_limits(&mut self, value: &Record<JsString, Any>) {
        self.inner.set("requiredLimits", value);
    }
}
impl GPUDeviceDescriptor {
    /// Getter of the `defaultQueue` attribute.
    pub fn default_queue(&self) -> GPUQueueDescriptor {
        self.inner.get("defaultQueue").as_::<GPUQueueDescriptor>()
    }

    /// Setter of the `defaultQueue` attribute.
    pub fn set_default_queue(&mut self, value: &GPUQueueDescriptor) {
        self.inner.set("defaultQueue", value);
    }
}

use super::*;

/// The GPUBufferBindingLayout dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct GPUBufferBindingLayout {
    inner: Any,
}

impl FromVal for GPUBufferBindingLayout {
    fn from_val(v: &Any) -> Self {
        GPUBufferBindingLayout { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for GPUBufferBindingLayout {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for GPUBufferBindingLayout {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for GPUBufferBindingLayout {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for GPUBufferBindingLayout {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<GPUBufferBindingLayout> for Any {
    fn from(s: GPUBufferBindingLayout) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&GPUBufferBindingLayout> for Any {
    fn from(s: &GPUBufferBindingLayout) -> Any {
        s.inner.clone()
    }
}

impl GPUBufferBindingLayout {
    /// Getter of the `type` attribute.
    pub fn type_(&self) -> GPUBufferBindingType {
        self.inner.get("type").as_::<GPUBufferBindingType>()
    }

    /// Setter of the `type` attribute.
    pub fn set_type_(&mut self, value: &GPUBufferBindingType) {
        self.inner.set("type", value);
    }
}
impl GPUBufferBindingLayout {
    /// Getter of the `hasDynamicOffset` attribute.
    pub fn has_dynamic_offset(&self) -> bool {
        self.inner.get("hasDynamicOffset").as_::<bool>()
    }

    /// Setter of the `hasDynamicOffset` attribute.
    pub fn set_has_dynamic_offset(&mut self, value: bool) {
        self.inner.set("hasDynamicOffset", value);
    }
}
impl GPUBufferBindingLayout {
    /// Getter of the `minBindingSize` attribute.
    pub fn min_binding_size(&self) -> Any {
        self.inner.get("minBindingSize").as_::<Any>()
    }

    /// Setter of the `minBindingSize` attribute.
    pub fn set_min_binding_size(&mut self, value: &Any) {
        self.inner.set("minBindingSize", value);
    }
}

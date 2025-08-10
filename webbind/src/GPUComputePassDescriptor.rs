use super::*;

/// The GPUComputePassDescriptor dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct GPUComputePassDescriptor {
    inner: Any,
}

impl FromVal for GPUComputePassDescriptor {
    fn from_val(v: &Any) -> Self {
        GPUComputePassDescriptor { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for GPUComputePassDescriptor {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for GPUComputePassDescriptor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for GPUComputePassDescriptor {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for GPUComputePassDescriptor {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<GPUComputePassDescriptor> for Any {
    fn from(s: GPUComputePassDescriptor) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&GPUComputePassDescriptor> for Any {
    fn from(s: &GPUComputePassDescriptor) -> Any {
        s.inner.clone()
    }
}

impl GPUComputePassDescriptor {
    /// Getter of the `timestampWrites` attribute.
    pub fn timestamp_writes(&self) -> GPUComputePassTimestampWrites {
        self.inner
            .get("timestampWrites")
            .as_::<GPUComputePassTimestampWrites>()
    }

    /// Setter of the `timestampWrites` attribute.
    pub fn set_timestamp_writes(&mut self, value: &GPUComputePassTimestampWrites) {
        self.inner.set("timestampWrites", value);
    }
}

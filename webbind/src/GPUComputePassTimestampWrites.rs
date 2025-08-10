use super::*;

/// The GPUComputePassTimestampWrites dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct GPUComputePassTimestampWrites {
    inner: Any,
}

impl FromVal for GPUComputePassTimestampWrites {
    fn from_val(v: &Any) -> Self {
        GPUComputePassTimestampWrites { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for GPUComputePassTimestampWrites {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for GPUComputePassTimestampWrites {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for GPUComputePassTimestampWrites {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for GPUComputePassTimestampWrites {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<GPUComputePassTimestampWrites> for Any {
    fn from(s: GPUComputePassTimestampWrites) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&GPUComputePassTimestampWrites> for Any {
    fn from(s: &GPUComputePassTimestampWrites) -> Any {
        s.inner.clone()
    }
}

impl GPUComputePassTimestampWrites {
    /// Getter of the `querySet` attribute.
    pub fn query_set(&self) -> GPUQuerySet {
        self.inner.get("querySet").as_::<GPUQuerySet>()
    }

    /// Setter of the `querySet` attribute.
    pub fn set_query_set(&mut self, value: &GPUQuerySet) {
        self.inner.set("querySet", value);
    }
}
impl GPUComputePassTimestampWrites {
    /// Getter of the `beginningOfPassWriteIndex` attribute.
    pub fn beginning_of_pass_write_index(&self) -> Any {
        self.inner.get("beginningOfPassWriteIndex").as_::<Any>()
    }

    /// Setter of the `beginningOfPassWriteIndex` attribute.
    pub fn set_beginning_of_pass_write_index(&mut self, value: &Any) {
        self.inner.set("beginningOfPassWriteIndex", value);
    }
}
impl GPUComputePassTimestampWrites {
    /// Getter of the `endOfPassWriteIndex` attribute.
    pub fn end_of_pass_write_index(&self) -> Any {
        self.inner.get("endOfPassWriteIndex").as_::<Any>()
    }

    /// Setter of the `endOfPassWriteIndex` attribute.
    pub fn set_end_of_pass_write_index(&mut self, value: &Any) {
        self.inner.set("endOfPassWriteIndex", value);
    }
}

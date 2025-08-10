use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct GPURenderPassTimestampWrites {
    inner: Any,
}
impl FromVal for GPURenderPassTimestampWrites {
    fn from_val(v: &Any) -> Self {
        GPURenderPassTimestampWrites { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for GPURenderPassTimestampWrites {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for GPURenderPassTimestampWrites {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for GPURenderPassTimestampWrites {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for GPURenderPassTimestampWrites {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<GPURenderPassTimestampWrites> for Any {
    fn from(s: GPURenderPassTimestampWrites) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&GPURenderPassTimestampWrites> for Any {
    fn from(s: &GPURenderPassTimestampWrites) -> Any {
        s.inner.clone()
    }
}

impl GPURenderPassTimestampWrites {
    pub fn query_set(&self) -> GPUQuerySet {
        self.inner.get("querySet").as_::<GPUQuerySet>()
    }

    pub fn set_query_set(&mut self, value: &GPUQuerySet) {
        self.inner.set("querySet", value);
    }
}
impl GPURenderPassTimestampWrites {
    pub fn beginning_of_pass_write_index(&self) -> Any {
        self.inner.get("beginningOfPassWriteIndex").as_::<Any>()
    }

    pub fn set_beginning_of_pass_write_index(&mut self, value: &Any) {
        self.inner.set("beginningOfPassWriteIndex", value);
    }
}
impl GPURenderPassTimestampWrites {
    pub fn end_of_pass_write_index(&self) -> Any {
        self.inner.get("endOfPassWriteIndex").as_::<Any>()
    }

    pub fn set_end_of_pass_write_index(&mut self, value: &Any) {
        self.inner.set("endOfPassWriteIndex", value);
    }
}

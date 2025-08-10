use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct GPUVertexBufferLayout {
    inner: Any,
}
impl FromVal for GPUVertexBufferLayout {
    fn from_val(v: &Any) -> Self {
        GPUVertexBufferLayout { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for GPUVertexBufferLayout {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for GPUVertexBufferLayout {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for GPUVertexBufferLayout {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for GPUVertexBufferLayout {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<GPUVertexBufferLayout> for Any {
    fn from(s: GPUVertexBufferLayout) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&GPUVertexBufferLayout> for Any {
    fn from(s: &GPUVertexBufferLayout) -> Any {
        s.inner.clone()
    }
}

impl GPUVertexBufferLayout {
    pub fn array_stride(&self) -> Any {
        self.inner.get("arrayStride").as_::<Any>()
    }

    pub fn set_array_stride(&mut self, value: &Any) {
        self.inner.set("arrayStride", value);
    }
}
impl GPUVertexBufferLayout {
    pub fn step_mode(&self) -> GPUVertexStepMode {
        self.inner.get("stepMode").as_::<GPUVertexStepMode>()
    }

    pub fn set_step_mode(&mut self, value: &GPUVertexStepMode) {
        self.inner.set("stepMode", value);
    }
}
impl GPUVertexBufferLayout {
    pub fn attributes(&self) -> TypedArray<GPUVertexAttribute> {
        self.inner
            .get("attributes")
            .as_::<TypedArray<GPUVertexAttribute>>()
    }

    pub fn set_attributes(&mut self, value: &TypedArray<GPUVertexAttribute>) {
        self.inner.set("attributes", value);
    }
}

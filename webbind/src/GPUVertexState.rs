use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct GPUVertexState {
    inner: Any,
}
impl FromVal for GPUVertexState {
    fn from_val(v: &Any) -> Self {
        GPUVertexState { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for GPUVertexState {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for GPUVertexState {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for GPUVertexState {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for GPUVertexState {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<GPUVertexState> for Any {
    fn from(s: GPUVertexState) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&GPUVertexState> for Any {
    fn from(s: &GPUVertexState) -> Any {
        s.inner.clone()
    }
}

impl GPUVertexState {
    pub fn buffers(&self) -> TypedArray<GPUVertexBufferLayout> {
        self.inner
            .get("buffers")
            .as_::<TypedArray<GPUVertexBufferLayout>>()
    }

    pub fn set_buffers(&mut self, value: &TypedArray<GPUVertexBufferLayout>) {
        self.inner.set("buffers", value);
    }
}

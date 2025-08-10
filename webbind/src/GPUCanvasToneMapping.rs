use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct GPUCanvasToneMapping {
    inner: Any,
}
impl FromVal for GPUCanvasToneMapping {
    fn from_val(v: &Any) -> Self {
        GPUCanvasToneMapping { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for GPUCanvasToneMapping {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for GPUCanvasToneMapping {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for GPUCanvasToneMapping {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for GPUCanvasToneMapping {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<GPUCanvasToneMapping> for Any {
    fn from(s: GPUCanvasToneMapping) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&GPUCanvasToneMapping> for Any {
    fn from(s: &GPUCanvasToneMapping) -> Any {
        s.inner.clone()
    }
}

impl GPUCanvasToneMapping {
    pub fn mode(&self) -> GPUCanvasToneMappingMode {
        self.inner.get("mode").as_::<GPUCanvasToneMappingMode>()
    }

    pub fn set_mode(&mut self, value: &GPUCanvasToneMappingMode) {
        self.inner.set("mode", value);
    }
}

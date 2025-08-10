use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct GPUExternalTextureBindingLayout {
    inner: Any,
}
impl FromVal for GPUExternalTextureBindingLayout {
    fn from_val(v: &Any) -> Self {
        GPUExternalTextureBindingLayout { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for GPUExternalTextureBindingLayout {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for GPUExternalTextureBindingLayout {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for GPUExternalTextureBindingLayout {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for GPUExternalTextureBindingLayout {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<GPUExternalTextureBindingLayout> for Any {
    fn from(s: GPUExternalTextureBindingLayout) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&GPUExternalTextureBindingLayout> for Any {
    fn from(s: &GPUExternalTextureBindingLayout) -> Any {
        s.inner.clone()
    }
}

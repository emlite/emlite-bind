use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GPUOutOfMemoryError {
    inner: GPUError,
}
impl FromVal for GPUOutOfMemoryError {
    fn from_val(v: &emlite::Val) -> Self {
        GPUOutOfMemoryError {
            inner: GPUError::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for GPUOutOfMemoryError {
    type Target = GPUError;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for GPUOutOfMemoryError {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<GPUOutOfMemoryError> for emlite::Val {
    fn from(s: GPUOutOfMemoryError) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl GPUOutOfMemoryError {
    pub fn new(message: jsbind::DOMString) -> GPUOutOfMemoryError {
        Self {
            inner: emlite::Val::global("GPUOutOfMemoryError")
                .new(&[message.into()])
                .as_::<GPUError>(),
        }
    }
}

use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GPUValidationError {
    inner: GPUError,
}
impl FromVal for GPUValidationError {
    fn from_val(v: &emlite::Val) -> Self {
        GPUValidationError {
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
impl core::ops::Deref for GPUValidationError {
    type Target = GPUError;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for GPUValidationError {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<GPUValidationError> for emlite::Val {
    fn from(s: GPUValidationError) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl GPUValidationError {
    pub fn new(message: jsbind::DOMString) -> GPUValidationError {
        Self {
            inner: emlite::Val::global("GPUValidationError")
                .new(&[message.into()])
                .as_::<GPUError>(),
        }
    }
}

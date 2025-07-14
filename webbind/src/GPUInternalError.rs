use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GPUInternalError {
    inner: GPUError,
}
impl FromVal for GPUInternalError {
    fn from_val(v: &emlite::Val) -> Self {
        GPUInternalError {
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
impl core::ops::Deref for GPUInternalError {
    type Target = GPUError;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for GPUInternalError {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<GPUInternalError> for emlite::Val {
    fn from(s: GPUInternalError) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl GPUInternalError {
    pub fn new(message: jsbind::DOMString) -> GPUInternalError {
        Self {
            inner: emlite::Val::global("GPUInternalError")
                .new(&[message.into()])
                .as_::<GPUError>(),
        }
    }
}

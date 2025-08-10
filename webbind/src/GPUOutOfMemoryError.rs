use super::*;

/// The GPUOutOfMemoryError class.
/// [`GPUOutOfMemoryError`](https://developer.mozilla.org/en-US/docs/Web/API/GPUOutOfMemoryError)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct GPUOutOfMemoryError {
    inner: GPUError,
}

impl FromVal for GPUOutOfMemoryError {
    fn from_val(v: &Any) -> Self {
        GPUOutOfMemoryError {
            inner: GPUError::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
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

impl AsRef<Any> for GPUOutOfMemoryError {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for GPUOutOfMemoryError {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<GPUOutOfMemoryError> for Any {
    fn from(s: GPUOutOfMemoryError) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&GPUOutOfMemoryError> for Any {
    fn from(s: &GPUOutOfMemoryError) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(GPUOutOfMemoryError);

impl GPUOutOfMemoryError {
    /// The `new GPUOutOfMemoryError(..)` constructor, creating a new GPUOutOfMemoryError instance
    pub fn new(message: &JsString) -> GPUOutOfMemoryError {
        Self {
            inner: Any::global("GPUOutOfMemoryError")
                .new(&[message.into()])
                .as_::<GPUError>(),
        }
    }
}

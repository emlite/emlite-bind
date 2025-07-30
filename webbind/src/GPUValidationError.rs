use super::*;

/// The GPUValidationError class.
/// [`GPUValidationError`](https://developer.mozilla.org/en-US/docs/Web/API/GPUValidationError)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct GPUValidationError {
    inner: GPUError,
}
impl FromVal for GPUValidationError {
    fn from_val(v: &Any) -> Self {
        GPUValidationError {
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
impl AsRef<Any> for GPUValidationError {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for GPUValidationError {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<GPUValidationError> for Any {
    fn from(s: GPUValidationError) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&GPUValidationError> for Any {
    fn from(s: &GPUValidationError) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(GPUValidationError);

impl GPUValidationError {
    /// The `new GPUValidationError(..)` constructor, creating a new GPUValidationError instance
    pub fn new(message: &JsString) -> GPUValidationError {
        Self {
            inner: Any::global("GPUValidationError")
                .new(&[message.into()])
                .as_::<GPUError>(),
        }
    }
}

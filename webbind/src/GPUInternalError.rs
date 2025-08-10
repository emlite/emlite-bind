use super::*;

/// The GPUInternalError class.
/// [`GPUInternalError`](https://developer.mozilla.org/en-US/docs/Web/API/GPUInternalError)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct GPUInternalError {
    inner: GPUError,
}

impl FromVal for GPUInternalError {
    fn from_val(v: &Any) -> Self {
        GPUInternalError {
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

impl AsRef<Any> for GPUInternalError {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for GPUInternalError {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<GPUInternalError> for Any {
    fn from(s: GPUInternalError) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&GPUInternalError> for Any {
    fn from(s: &GPUInternalError) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(GPUInternalError);

impl GPUInternalError {
    /// The `new GPUInternalError(..)` constructor, creating a new GPUInternalError instance
    pub fn new(message: &JsString) -> GPUInternalError {
        Self {
            inner: Any::global("GPUInternalError")
                .new(&[message.into()])
                .as_::<GPUError>(),
        }
    }
}

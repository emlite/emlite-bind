use super::*;

/// The GPUError class.
/// [`GPUError`](https://developer.mozilla.org/en-US/docs/Web/API/GPUError)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct GPUError {
    inner: Any,
}
impl FromVal for GPUError {
    fn from_val(v: &Any) -> Self {
        GPUError {
            inner: Any::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for GPUError {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for GPUError {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for GPUError {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for GPUError {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<GPUError> for Any {
    fn from(s: GPUError) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&GPUError> for Any {
    fn from(s: &GPUError) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(GPUError);

impl GPUError {
    /// Getter of the `message` attribute.
    /// [`GPUError.message`](https://developer.mozilla.org/en-US/docs/Web/API/GPUError/message)
    pub fn message(&self) -> DOMString {
        self.inner.get("message").as_::<DOMString>()
    }
}

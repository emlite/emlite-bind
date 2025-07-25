use super::*;

/// The WebGLUniformLocation class.
/// [`WebGLUniformLocation`](https://developer.mozilla.org/en-US/docs/Web/API/WebGLUniformLocation)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct WebGLUniformLocation {
    inner: Any,
}
impl FromVal for WebGLUniformLocation {
    fn from_val(v: &Any) -> Self {
        WebGLUniformLocation {
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
impl core::ops::Deref for WebGLUniformLocation {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for WebGLUniformLocation {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for WebGLUniformLocation {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for WebGLUniformLocation {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<WebGLUniformLocation> for Any {
    fn from(s: WebGLUniformLocation) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&WebGLUniformLocation> for Any {
    fn from(s: &WebGLUniformLocation) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(WebGLUniformLocation);

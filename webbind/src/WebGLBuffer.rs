use super::*;

/// The WebGLBuffer class.
/// [`WebGLBuffer`](https://developer.mozilla.org/en-US/docs/Web/API/WebGLBuffer)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct WebGLBuffer {
    inner: WebGLObject,
}

impl FromVal for WebGLBuffer {
    fn from_val(v: &Any) -> Self {
        WebGLBuffer {
            inner: WebGLObject::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for WebGLBuffer {
    type Target = WebGLObject;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for WebGLBuffer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for WebGLBuffer {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for WebGLBuffer {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<WebGLBuffer> for Any {
    fn from(s: WebGLBuffer) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&WebGLBuffer> for Any {
    fn from(s: &WebGLBuffer) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(WebGLBuffer);

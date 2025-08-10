use super::*;

/// The WebGLFramebuffer class.
/// [`WebGLFramebuffer`](https://developer.mozilla.org/en-US/docs/Web/API/WebGLFramebuffer)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct WebGLFramebuffer {
    inner: WebGLObject,
}

impl FromVal for WebGLFramebuffer {
    fn from_val(v: &Any) -> Self {
        WebGLFramebuffer {
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

impl core::ops::Deref for WebGLFramebuffer {
    type Target = WebGLObject;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for WebGLFramebuffer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for WebGLFramebuffer {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for WebGLFramebuffer {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<WebGLFramebuffer> for Any {
    fn from(s: WebGLFramebuffer) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&WebGLFramebuffer> for Any {
    fn from(s: &WebGLFramebuffer) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(WebGLFramebuffer);

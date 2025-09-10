use super::*;

/// The WebGLVertexArrayObjectOES class.
/// [`WebGLVertexArrayObjectOES`](https://developer.mozilla.org/en-US/docs/Web/API/WebGLVertexArrayObjectOES)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct WebGLVertexArrayObjectOES {
    inner: WebGLObject,
}

impl FromVal for WebGLVertexArrayObjectOES {
    fn from_val(v: &Any) -> Self {
        WebGLVertexArrayObjectOES {
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

impl core::ops::Deref for WebGLVertexArrayObjectOES {
    type Target = WebGLObject;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for WebGLVertexArrayObjectOES {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for WebGLVertexArrayObjectOES {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for WebGLVertexArrayObjectOES {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<WebGLVertexArrayObjectOES> for Any {
    fn from(s: WebGLVertexArrayObjectOES) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&WebGLVertexArrayObjectOES> for Any {
    fn from(s: &WebGLVertexArrayObjectOES) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(WebGLVertexArrayObjectOES);

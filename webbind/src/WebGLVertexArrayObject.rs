use super::*;




/// The WebGLVertexArrayObject class.
/// [`WebGLVertexArrayObject`](https://developer.mozilla.org/en-US/docs/Web/API/WebGLVertexArrayObject)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct WebGLVertexArrayObject {
    inner: WebGLObject,
}

impl FromVal for WebGLVertexArrayObject {
    fn from_val(v: &Any) -> Self {
        WebGLVertexArrayObject { inner: WebGLObject::from_val(v) }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for WebGLVertexArrayObject {
    type Target = WebGLObject;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for WebGLVertexArrayObject {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for WebGLVertexArrayObject {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for WebGLVertexArrayObject {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<WebGLVertexArrayObject> for Any {
    fn from(s: WebGLVertexArrayObject) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&WebGLVertexArrayObject> for Any {
    fn from(s: &WebGLVertexArrayObject) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(WebGLVertexArrayObject);



use super::*;




/// The WebGLProgram class.
/// [`WebGLProgram`](https://developer.mozilla.org/en-US/docs/Web/API/WebGLProgram)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct WebGLProgram {
    inner: WebGLObject,
}

impl FromVal for WebGLProgram {
    fn from_val(v: &Any) -> Self {
        WebGLProgram { inner: WebGLObject::from_val(v) }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for WebGLProgram {
    type Target = WebGLObject;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for WebGLProgram {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for WebGLProgram {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for WebGLProgram {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<WebGLProgram> for Any {
    fn from(s: WebGLProgram) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&WebGLProgram> for Any {
    fn from(s: &WebGLProgram) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(WebGLProgram);



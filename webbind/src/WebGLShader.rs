use super::*;




/// The WebGLShader class.
/// [`WebGLShader`](https://developer.mozilla.org/en-US/docs/Web/API/WebGLShader)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct WebGLShader {
    inner: WebGLObject,
}

impl FromVal for WebGLShader {
    fn from_val(v: &Any) -> Self {
        WebGLShader { inner: WebGLObject::from_val(v) }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for WebGLShader {
    type Target = WebGLObject;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for WebGLShader {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for WebGLShader {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for WebGLShader {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<WebGLShader> for Any {
    fn from(s: WebGLShader) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&WebGLShader> for Any {
    fn from(s: &WebGLShader) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(WebGLShader);



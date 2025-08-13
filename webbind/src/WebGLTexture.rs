use super::*;




/// The WebGLTexture class.
/// [`WebGLTexture`](https://developer.mozilla.org/en-US/docs/Web/API/WebGLTexture)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct WebGLTexture {
    inner: WebGLObject,
}

impl FromVal for WebGLTexture {
    fn from_val(v: &Any) -> Self {
        WebGLTexture { inner: WebGLObject::from_val(v) }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for WebGLTexture {
    type Target = WebGLObject;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for WebGLTexture {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for WebGLTexture {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for WebGLTexture {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<WebGLTexture> for Any {
    fn from(s: WebGLTexture) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&WebGLTexture> for Any {
    fn from(s: &WebGLTexture) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(WebGLTexture);



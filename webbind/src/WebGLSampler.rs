use super::*;




/// The WebGLSampler class.
/// [`WebGLSampler`](https://developer.mozilla.org/en-US/docs/Web/API/WebGLSampler)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct WebGLSampler {
    inner: WebGLObject,
}

impl FromVal for WebGLSampler {
    fn from_val(v: &Any) -> Self {
        WebGLSampler { inner: WebGLObject::from_val(v) }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for WebGLSampler {
    type Target = WebGLObject;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for WebGLSampler {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for WebGLSampler {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for WebGLSampler {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<WebGLSampler> for Any {
    fn from(s: WebGLSampler) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&WebGLSampler> for Any {
    fn from(s: &WebGLSampler) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(WebGLSampler);



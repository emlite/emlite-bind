use super::*;




/// The WebGLContextAttributes dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct WebGLContextAttributes {
    inner: Any,
}

impl FromVal for WebGLContextAttributes {
    fn from_val(v: &Any) -> Self {
        WebGLContextAttributes { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for WebGLContextAttributes {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for WebGLContextAttributes {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for WebGLContextAttributes {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for WebGLContextAttributes {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<WebGLContextAttributes> for Any {
    fn from(s: WebGLContextAttributes) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&WebGLContextAttributes> for Any {
    fn from(s: &WebGLContextAttributes) -> Any {
        s.inner.clone()
    }
}

impl WebGLContextAttributes {
    /// Getter of the `xrCompatible` attribute.
    pub fn xr_compatible(&self) -> bool {
        self.inner.get("xrCompatible").as_::<bool>()
    }

    /// Setter of the `xrCompatible` attribute.
    pub fn set_xr_compatible(&mut self, value: bool) {
        self.inner.set("xrCompatible", value);
    }
}

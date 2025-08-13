use super::*;




/// The WEBGL_debug_shaders class.
/// [`WEBGL_debug_shaders`](https://developer.mozilla.org/en-US/docs/Web/API/WEBGL_debug_shaders)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct WEBGL_debug_shaders {
    inner: Any,
}

impl FromVal for WEBGL_debug_shaders {
    fn from_val(v: &Any) -> Self {
        WEBGL_debug_shaders { inner: Any::from_val(v) }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for WEBGL_debug_shaders {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for WEBGL_debug_shaders {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for WEBGL_debug_shaders {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for WEBGL_debug_shaders {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<WEBGL_debug_shaders> for Any {
    fn from(s: WEBGL_debug_shaders) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&WEBGL_debug_shaders> for Any {
    fn from(s: &WEBGL_debug_shaders) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(WEBGL_debug_shaders);


impl WEBGL_debug_shaders {
    /// The getTranslatedShaderSource method.
    /// [`WEBGL_debug_shaders.getTranslatedShaderSource`](https://developer.mozilla.org/en-US/docs/Web/API/WEBGL_debug_shaders/getTranslatedShaderSource)
    pub fn get_translated_shader_source(&self, shader: &WebGLShader) -> JsString {
        self.inner.call("getTranslatedShaderSource", &[shader.into(), ]).as_::<JsString>()
    }
}

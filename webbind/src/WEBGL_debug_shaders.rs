use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct WEBGL_debug_shaders {
    inner: emlite::Val,
}
impl FromVal for WEBGL_debug_shaders {
    fn from_val(v: &emlite::Val) -> Self {
        WEBGL_debug_shaders {
            inner: emlite::Val::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for WEBGL_debug_shaders {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for WEBGL_debug_shaders {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for WEBGL_debug_shaders {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for WEBGL_debug_shaders {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<WEBGL_debug_shaders> for emlite::Val {
    fn from(s: WEBGL_debug_shaders) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(WEBGL_debug_shaders);

impl WEBGL_debug_shaders {
    pub fn get_translated_shader_source(&self, shader: WebGLShader) -> jsbind::DOMString {
        self.inner
            .call("getTranslatedShaderSource", &[shader.into()])
            .as_::<jsbind::DOMString>()
    }
}

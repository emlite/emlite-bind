use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct WEBGL_compressed_texture_astc {
    inner: emlite::Val,
}
impl FromVal for WEBGL_compressed_texture_astc {
    fn from_val(v: &emlite::Val) -> Self {
        WEBGL_compressed_texture_astc {
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
impl core::ops::Deref for WEBGL_compressed_texture_astc {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for WEBGL_compressed_texture_astc {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<WEBGL_compressed_texture_astc> for emlite::Val {
    fn from(s: WEBGL_compressed_texture_astc) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl WEBGL_compressed_texture_astc {
    pub fn get_supported_profiles(&self) -> jsbind::Sequence<jsbind::DOMString> {
        self.inner
            .call("getSupportedProfiles", &[])
            .as_::<jsbind::Sequence<jsbind::DOMString>>()
    }
}

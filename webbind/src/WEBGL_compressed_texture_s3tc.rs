use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct WEBGL_compressed_texture_s3tc {
    inner: emlite::Val,
}
impl FromVal for WEBGL_compressed_texture_s3tc {
    fn from_val(v: &emlite::Val) -> Self {
        WEBGL_compressed_texture_s3tc {
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
impl core::ops::Deref for WEBGL_compressed_texture_s3tc {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for WEBGL_compressed_texture_s3tc {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<WEBGL_compressed_texture_s3tc> for emlite::Val {
    fn from(s: WEBGL_compressed_texture_s3tc) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

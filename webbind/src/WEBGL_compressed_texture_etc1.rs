use super::*;

#[derive(Clone, Debug)]
pub struct WEBGL_compressed_texture_etc1 {
    inner: emlite::Val,
}
impl FromVal for WEBGL_compressed_texture_etc1 {
    fn from_val(v: &emlite::Val) -> Self {
        WEBGL_compressed_texture_etc1 {
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
impl std::ops::Deref for WEBGL_compressed_texture_etc1 {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for WEBGL_compressed_texture_etc1 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<WEBGL_compressed_texture_etc1> for emlite::Val {
    fn from(s: WEBGL_compressed_texture_etc1) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

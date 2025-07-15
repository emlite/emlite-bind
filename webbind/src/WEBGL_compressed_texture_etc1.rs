use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
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
impl core::ops::Deref for WEBGL_compressed_texture_etc1 {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for WEBGL_compressed_texture_etc1 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for WEBGL_compressed_texture_etc1 {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for WEBGL_compressed_texture_etc1 {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<WEBGL_compressed_texture_etc1> for emlite::Val {
    fn from(s: WEBGL_compressed_texture_etc1) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&WEBGL_compressed_texture_etc1> for emlite::Val {
    fn from(s: &WEBGL_compressed_texture_etc1) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(WEBGL_compressed_texture_etc1);

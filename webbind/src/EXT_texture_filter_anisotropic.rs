use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct EXT_texture_filter_anisotropic {
    inner: emlite::Val,
}
impl FromVal for EXT_texture_filter_anisotropic {
    fn from_val(v: &emlite::Val) -> Self {
        EXT_texture_filter_anisotropic {
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
impl core::ops::Deref for EXT_texture_filter_anisotropic {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for EXT_texture_filter_anisotropic {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<EXT_texture_filter_anisotropic> for emlite::Val {
    fn from(s: EXT_texture_filter_anisotropic) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

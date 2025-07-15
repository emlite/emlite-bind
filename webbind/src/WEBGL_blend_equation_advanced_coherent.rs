use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct WEBGL_blend_equation_advanced_coherent {
    inner: emlite::Val,
}
impl FromVal for WEBGL_blend_equation_advanced_coherent {
    fn from_val(v: &emlite::Val) -> Self {
        WEBGL_blend_equation_advanced_coherent {
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
impl core::ops::Deref for WEBGL_blend_equation_advanced_coherent {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for WEBGL_blend_equation_advanced_coherent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for WEBGL_blend_equation_advanced_coherent {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for WEBGL_blend_equation_advanced_coherent {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<WEBGL_blend_equation_advanced_coherent> for emlite::Val {
    fn from(s: WEBGL_blend_equation_advanced_coherent) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&WEBGL_blend_equation_advanced_coherent> for emlite::Val {
    fn from(s: &WEBGL_blend_equation_advanced_coherent) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(WEBGL_blend_equation_advanced_coherent);

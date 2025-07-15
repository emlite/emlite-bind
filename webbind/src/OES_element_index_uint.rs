use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct OES_element_index_uint {
    inner: emlite::Val,
}
impl FromVal for OES_element_index_uint {
    fn from_val(v: &emlite::Val) -> Self {
        OES_element_index_uint {
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
impl core::ops::Deref for OES_element_index_uint {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for OES_element_index_uint {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for OES_element_index_uint {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for OES_element_index_uint {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<OES_element_index_uint> for emlite::Val {
    fn from(s: OES_element_index_uint) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&OES_element_index_uint> for emlite::Val {
    fn from(s: &OES_element_index_uint) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(OES_element_index_uint);

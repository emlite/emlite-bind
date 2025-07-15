use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct EXT_color_buffer_half_float {
    inner: emlite::Val,
}
impl FromVal for EXT_color_buffer_half_float {
    fn from_val(v: &emlite::Val) -> Self {
        EXT_color_buffer_half_float {
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
impl core::ops::Deref for EXT_color_buffer_half_float {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for EXT_color_buffer_half_float {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for EXT_color_buffer_half_float {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for EXT_color_buffer_half_float {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<EXT_color_buffer_half_float> for emlite::Val {
    fn from(s: EXT_color_buffer_half_float) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&EXT_color_buffer_half_float> for emlite::Val {
    fn from(s: &EXT_color_buffer_half_float) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(EXT_color_buffer_half_float);

use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct WEBGL_clip_cull_distance {
    inner: emlite::Val,
}
impl FromVal for WEBGL_clip_cull_distance {
    fn from_val(v: &emlite::Val) -> Self {
        WEBGL_clip_cull_distance {
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
impl core::ops::Deref for WEBGL_clip_cull_distance {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for WEBGL_clip_cull_distance {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<WEBGL_clip_cull_distance> for emlite::Val {
    fn from(s: WEBGL_clip_cull_distance) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

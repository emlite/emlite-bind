use super::*;

#[derive(Clone, Debug)]
pub struct WEBGL_debug_renderer_info {
    inner: emlite::Val,
}
impl FromVal for WEBGL_debug_renderer_info {
    fn from_val(v: &emlite::Val) -> Self {
        WEBGL_debug_renderer_info {
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
impl std::ops::Deref for WEBGL_debug_renderer_info {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for WEBGL_debug_renderer_info {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<WEBGL_debug_renderer_info> for emlite::Val {
    fn from(s: WEBGL_debug_renderer_info) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

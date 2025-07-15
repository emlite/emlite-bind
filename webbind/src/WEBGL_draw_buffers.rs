use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct WEBGL_draw_buffers {
    inner: emlite::Val,
}
impl FromVal for WEBGL_draw_buffers {
    fn from_val(v: &emlite::Val) -> Self {
        WEBGL_draw_buffers {
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
impl core::ops::Deref for WEBGL_draw_buffers {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for WEBGL_draw_buffers {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for WEBGL_draw_buffers {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for WEBGL_draw_buffers {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<WEBGL_draw_buffers> for emlite::Val {
    fn from(s: WEBGL_draw_buffers) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&WEBGL_draw_buffers> for emlite::Val {
    fn from(s: &WEBGL_draw_buffers) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(WEBGL_draw_buffers);

impl WEBGL_draw_buffers {
    pub fn draw_buffers_webgl(&self, buffers: &Sequence<Any>) -> Undefined {
        self.inner
            .call("drawBuffersWEBGL", &[buffers.into()])
            .as_::<Undefined>()
    }
}

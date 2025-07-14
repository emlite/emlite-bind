use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct WEBGL_provoking_vertex {
    inner: emlite::Val,
}
impl FromVal for WEBGL_provoking_vertex {
    fn from_val(v: &emlite::Val) -> Self {
        WEBGL_provoking_vertex {
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
impl core::ops::Deref for WEBGL_provoking_vertex {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for WEBGL_provoking_vertex {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for WEBGL_provoking_vertex {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for WEBGL_provoking_vertex {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<WEBGL_provoking_vertex> for emlite::Val {
    fn from(s: WEBGL_provoking_vertex) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(WEBGL_provoking_vertex);

impl WEBGL_provoking_vertex {
    pub fn provoking_vertex_webgl(&self, provoke_mode: jsbind::Any) -> jsbind::Undefined {
        self.inner
            .call("provokingVertexWEBGL", &[provoke_mode.into()])
            .as_::<jsbind::Undefined>()
    }
}

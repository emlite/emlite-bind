use super::*;

#[derive(Clone, Debug)]
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
impl std::ops::Deref for WEBGL_provoking_vertex {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for WEBGL_provoking_vertex {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<WEBGL_provoking_vertex> for emlite::Val {
    fn from(s: WEBGL_provoking_vertex) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl WEBGL_provoking_vertex {
    pub fn provoking_vertex_webgl(&self, provoke_mode: jsbind::Any) -> jsbind::Undefined {
        self.inner
            .call("provokingVertexWEBGL", &[provoke_mode.into()])
            .as_::<jsbind::Undefined>()
    }
}

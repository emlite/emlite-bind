use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct EXT_disjoint_timer_query_webgl2 {
    inner: emlite::Val,
}
impl FromVal for EXT_disjoint_timer_query_webgl2 {
    fn from_val(v: &emlite::Val) -> Self {
        EXT_disjoint_timer_query_webgl2 {
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
impl core::ops::Deref for EXT_disjoint_timer_query_webgl2 {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for EXT_disjoint_timer_query_webgl2 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<EXT_disjoint_timer_query_webgl2> for emlite::Val {
    fn from(s: EXT_disjoint_timer_query_webgl2) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl EXT_disjoint_timer_query_webgl2 {
    pub fn query_counter_ext(&self, query: WebGLQuery, target: jsbind::Any) -> jsbind::Undefined {
        self.inner
            .call("queryCounterEXT", &[query.into(), target.into()])
            .as_::<jsbind::Undefined>()
    }
}

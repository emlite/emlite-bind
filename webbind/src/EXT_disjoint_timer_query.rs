use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct EXT_disjoint_timer_query {
    inner: emlite::Val,
}
impl FromVal for EXT_disjoint_timer_query {
    fn from_val(v: &emlite::Val) -> Self {
        EXT_disjoint_timer_query {
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
impl core::ops::Deref for EXT_disjoint_timer_query {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for EXT_disjoint_timer_query {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<EXT_disjoint_timer_query> for emlite::Val {
    fn from(s: EXT_disjoint_timer_query) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl EXT_disjoint_timer_query {
    pub fn create_query_ext(&self) -> WebGLTimerQueryEXT {
        self.inner
            .call("createQueryEXT", &[])
            .as_::<WebGLTimerQueryEXT>()
    }
}
impl EXT_disjoint_timer_query {
    pub fn delete_query_ext(&self, query: WebGLTimerQueryEXT) -> jsbind::Undefined {
        self.inner
            .call("deleteQueryEXT", &[query.into()])
            .as_::<jsbind::Undefined>()
    }
}
impl EXT_disjoint_timer_query {
    pub fn is_query_ext(&self, query: WebGLTimerQueryEXT) -> bool {
        self.inner.call("isQueryEXT", &[query.into()]).as_::<bool>()
    }
}
impl EXT_disjoint_timer_query {
    pub fn begin_query_ext(
        &self,
        target: jsbind::Any,
        query: WebGLTimerQueryEXT,
    ) -> jsbind::Undefined {
        self.inner
            .call("beginQueryEXT", &[target.into(), query.into()])
            .as_::<jsbind::Undefined>()
    }
}
impl EXT_disjoint_timer_query {
    pub fn end_query_ext(&self, target: jsbind::Any) -> jsbind::Undefined {
        self.inner
            .call("endQueryEXT", &[target.into()])
            .as_::<jsbind::Undefined>()
    }
}
impl EXT_disjoint_timer_query {
    pub fn query_counter_ext(
        &self,
        query: WebGLTimerQueryEXT,
        target: jsbind::Any,
    ) -> jsbind::Undefined {
        self.inner
            .call("queryCounterEXT", &[query.into(), target.into()])
            .as_::<jsbind::Undefined>()
    }
}
impl EXT_disjoint_timer_query {
    pub fn get_query_ext(&self, target: jsbind::Any, pname: jsbind::Any) -> jsbind::Any {
        self.inner
            .call("getQueryEXT", &[target.into(), pname.into()])
            .as_::<jsbind::Any>()
    }
}
impl EXT_disjoint_timer_query {
    pub fn get_query_object_ext(
        &self,
        query: WebGLTimerQueryEXT,
        pname: jsbind::Any,
    ) -> jsbind::Any {
        self.inner
            .call("getQueryObjectEXT", &[query.into(), pname.into()])
            .as_::<jsbind::Any>()
    }
}

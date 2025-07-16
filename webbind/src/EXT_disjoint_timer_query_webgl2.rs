use super::*;

/// The EXT_disjoint_timer_query_webgl2 class.
/// [`EXT_disjoint_timer_query_webgl2`](https://developer.mozilla.org/en-US/docs/Web/API/EXT_disjoint_timer_query_webgl2)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct EXT_disjoint_timer_query_webgl2 {
    inner: Any,
}
impl FromVal for EXT_disjoint_timer_query_webgl2 {
    fn from_val(v: &Any) -> Self {
        EXT_disjoint_timer_query_webgl2 {
            inner: Any::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for EXT_disjoint_timer_query_webgl2 {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for EXT_disjoint_timer_query_webgl2 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for EXT_disjoint_timer_query_webgl2 {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for EXT_disjoint_timer_query_webgl2 {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<EXT_disjoint_timer_query_webgl2> for Any {
    fn from(s: EXT_disjoint_timer_query_webgl2) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&EXT_disjoint_timer_query_webgl2> for Any {
    fn from(s: &EXT_disjoint_timer_query_webgl2) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(EXT_disjoint_timer_query_webgl2);

impl EXT_disjoint_timer_query_webgl2 {
    /// The queryCounterEXT method.
    /// [`EXT_disjoint_timer_query_webgl2.queryCounterEXT`](https://developer.mozilla.org/en-US/docs/Web/API/EXT_disjoint_timer_query_webgl2/queryCounterEXT)
    pub fn query_counter_ext(&self, query: &WebGLQuery, target: &Any) -> Undefined {
        self.inner
            .call("queryCounterEXT", &[query.into(), target.into()])
            .as_::<Undefined>()
    }
}

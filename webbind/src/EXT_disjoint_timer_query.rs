use super::*;

/// The EXT_disjoint_timer_query class.
/// [`EXT_disjoint_timer_query`](https://developer.mozilla.org/en-US/docs/Web/API/EXT_disjoint_timer_query)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct EXT_disjoint_timer_query {
    inner: Any,
}

impl FromVal for EXT_disjoint_timer_query {
    fn from_val(v: &Any) -> Self {
        EXT_disjoint_timer_query {
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

impl core::ops::Deref for EXT_disjoint_timer_query {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for EXT_disjoint_timer_query {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for EXT_disjoint_timer_query {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for EXT_disjoint_timer_query {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<EXT_disjoint_timer_query> for Any {
    fn from(s: EXT_disjoint_timer_query) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&EXT_disjoint_timer_query> for Any {
    fn from(s: &EXT_disjoint_timer_query) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(EXT_disjoint_timer_query);

impl EXT_disjoint_timer_query {
    /// The createQueryEXT method.
    /// [`EXT_disjoint_timer_query.createQueryEXT`](https://developer.mozilla.org/en-US/docs/Web/API/EXT_disjoint_timer_query/createQueryEXT)
    pub fn create_query_ext(&self) -> WebGLTimerQueryEXT {
        self.inner
            .call("createQueryEXT", &[])
            .as_::<WebGLTimerQueryEXT>()
    }
}
impl EXT_disjoint_timer_query {
    /// The deleteQueryEXT method.
    /// [`EXT_disjoint_timer_query.deleteQueryEXT`](https://developer.mozilla.org/en-US/docs/Web/API/EXT_disjoint_timer_query/deleteQueryEXT)
    pub fn delete_query_ext(&self, query: &WebGLTimerQueryEXT) -> Undefined {
        self.inner
            .call("deleteQueryEXT", &[query.into()])
            .as_::<Undefined>()
    }
}
impl EXT_disjoint_timer_query {
    /// The isQueryEXT method.
    /// [`EXT_disjoint_timer_query.isQueryEXT`](https://developer.mozilla.org/en-US/docs/Web/API/EXT_disjoint_timer_query/isQueryEXT)
    pub fn is_query_ext(&self, query: &WebGLTimerQueryEXT) -> bool {
        self.inner.call("isQueryEXT", &[query.into()]).as_::<bool>()
    }
}
impl EXT_disjoint_timer_query {
    /// The beginQueryEXT method.
    /// [`EXT_disjoint_timer_query.beginQueryEXT`](https://developer.mozilla.org/en-US/docs/Web/API/EXT_disjoint_timer_query/beginQueryEXT)
    pub fn begin_query_ext(&self, target: &Any, query: &WebGLTimerQueryEXT) -> Undefined {
        self.inner
            .call("beginQueryEXT", &[target.into(), query.into()])
            .as_::<Undefined>()
    }
}
impl EXT_disjoint_timer_query {
    /// The endQueryEXT method.
    /// [`EXT_disjoint_timer_query.endQueryEXT`](https://developer.mozilla.org/en-US/docs/Web/API/EXT_disjoint_timer_query/endQueryEXT)
    pub fn end_query_ext(&self, target: &Any) -> Undefined {
        self.inner
            .call("endQueryEXT", &[target.into()])
            .as_::<Undefined>()
    }
}
impl EXT_disjoint_timer_query {
    /// The queryCounterEXT method.
    /// [`EXT_disjoint_timer_query.queryCounterEXT`](https://developer.mozilla.org/en-US/docs/Web/API/EXT_disjoint_timer_query/queryCounterEXT)
    pub fn query_counter_ext(&self, query: &WebGLTimerQueryEXT, target: &Any) -> Undefined {
        self.inner
            .call("queryCounterEXT", &[query.into(), target.into()])
            .as_::<Undefined>()
    }
}
impl EXT_disjoint_timer_query {
    /// The getQueryEXT method.
    /// [`EXT_disjoint_timer_query.getQueryEXT`](https://developer.mozilla.org/en-US/docs/Web/API/EXT_disjoint_timer_query/getQueryEXT)
    pub fn get_query_ext(&self, target: &Any, pname: &Any) -> Any {
        self.inner
            .call("getQueryEXT", &[target.into(), pname.into()])
            .as_::<Any>()
    }
}
impl EXT_disjoint_timer_query {
    /// The getQueryObjectEXT method.
    /// [`EXT_disjoint_timer_query.getQueryObjectEXT`](https://developer.mozilla.org/en-US/docs/Web/API/EXT_disjoint_timer_query/getQueryObjectEXT)
    pub fn get_query_object_ext(&self, query: &WebGLTimerQueryEXT, pname: &Any) -> Any {
        self.inner
            .call("getQueryObjectEXT", &[query.into(), pname.into()])
            .as_::<Any>()
    }
}

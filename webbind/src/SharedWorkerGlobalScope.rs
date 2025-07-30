use super::*;

/// The SharedWorkerGlobalScope class.
/// [`SharedWorkerGlobalScope`](https://developer.mozilla.org/en-US/docs/Web/API/SharedWorkerGlobalScope)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct SharedWorkerGlobalScope {
    inner: WorkerGlobalScope,
}
impl FromVal for SharedWorkerGlobalScope {
    fn from_val(v: &Any) -> Self {
        SharedWorkerGlobalScope {
            inner: WorkerGlobalScope::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for SharedWorkerGlobalScope {
    type Target = WorkerGlobalScope;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for SharedWorkerGlobalScope {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for SharedWorkerGlobalScope {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for SharedWorkerGlobalScope {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<SharedWorkerGlobalScope> for Any {
    fn from(s: SharedWorkerGlobalScope) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&SharedWorkerGlobalScope> for Any {
    fn from(s: &SharedWorkerGlobalScope) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(SharedWorkerGlobalScope);

impl SharedWorkerGlobalScope {
    /// Getter of the `name` attribute.
    /// [`SharedWorkerGlobalScope.name`](https://developer.mozilla.org/en-US/docs/Web/API/SharedWorkerGlobalScope/name)
    pub fn name(&self) -> JsString {
        self.inner.get("name").as_::<JsString>()
    }
}
impl SharedWorkerGlobalScope {
    /// The close method.
    /// [`SharedWorkerGlobalScope.close`](https://developer.mozilla.org/en-US/docs/Web/API/SharedWorkerGlobalScope/close)
    pub fn close(&self) -> Undefined {
        self.inner.call("close", &[]).as_::<Undefined>()
    }
}
impl SharedWorkerGlobalScope {
    /// Getter of the `onconnect` attribute.
    /// [`SharedWorkerGlobalScope.onconnect`](https://developer.mozilla.org/en-US/docs/Web/API/SharedWorkerGlobalScope/onconnect)
    pub fn onconnect(&self) -> Any {
        self.inner.get("onconnect").as_::<Any>()
    }

    /// Setter of the `onconnect` attribute.
    /// [`SharedWorkerGlobalScope.onconnect`](https://developer.mozilla.org/en-US/docs/Web/API/SharedWorkerGlobalScope/onconnect)
    pub fn set_onconnect(&mut self, value: &Any) {
        self.inner.set("onconnect", value);
    }
}

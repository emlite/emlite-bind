use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct SharedWorkerGlobalScope {
    inner: WorkerGlobalScope,
}
impl FromVal for SharedWorkerGlobalScope {
    fn from_val(v: &emlite::Val) -> Self {
        SharedWorkerGlobalScope {
            inner: WorkerGlobalScope::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
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
impl AsRef<emlite::Val> for SharedWorkerGlobalScope {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for SharedWorkerGlobalScope {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<SharedWorkerGlobalScope> for emlite::Val {
    fn from(s: SharedWorkerGlobalScope) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(SharedWorkerGlobalScope);

impl SharedWorkerGlobalScope {
    pub fn name(&self) -> jsbind::DOMString {
        self.inner.get("name").as_::<jsbind::DOMString>()
    }
}
impl SharedWorkerGlobalScope {
    pub fn close(&self) -> jsbind::Undefined {
        self.inner.call("close", &[]).as_::<jsbind::Undefined>()
    }
}
impl SharedWorkerGlobalScope {
    pub fn onconnect(&self) -> jsbind::Any {
        self.inner.get("onconnect").as_::<jsbind::Any>()
    }

    pub fn set_onconnect(&mut self, value: jsbind::Any) {
        self.inner.set("onconnect", value);
    }
}

use super::*;

#[derive(Clone, Debug)]
pub struct WorkerGlobalScope {
    inner: EventTarget,
}
impl FromVal for WorkerGlobalScope {
    fn from_val(v: &emlite::Val) -> Self {
        WorkerGlobalScope {
            inner: EventTarget::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for WorkerGlobalScope {
    type Target = EventTarget;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for WorkerGlobalScope {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<WorkerGlobalScope> for emlite::Val {
    fn from(s: WorkerGlobalScope) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl WorkerGlobalScope {
    pub fn self_(&self) -> WorkerGlobalScope {
        self.inner.get("self").as_::<WorkerGlobalScope>()
    }
}
impl WorkerGlobalScope {
    pub fn location(&self) -> WorkerLocation {
        self.inner.get("location").as_::<WorkerLocation>()
    }
}
impl WorkerGlobalScope {
    pub fn navigator(&self) -> WorkerNavigator {
        self.inner.get("navigator").as_::<WorkerNavigator>()
    }
}
impl WorkerGlobalScope {
    pub fn import_scripts(&self, urls: jsbind::Any) -> jsbind::Undefined {
        self.inner
            .call("importScripts", &[urls.into()])
            .as_::<jsbind::Undefined>()
    }
}
impl WorkerGlobalScope {
    pub fn onerror(&self) -> jsbind::Any {
        self.inner.get("onerror").as_::<jsbind::Any>()
    }

    pub fn set_onerror(&mut self, value: jsbind::Any) {
        self.inner.set("onerror", value);
    }
}
impl WorkerGlobalScope {
    pub fn onlanguagechange(&self) -> jsbind::Any {
        self.inner.get("onlanguagechange").as_::<jsbind::Any>()
    }

    pub fn set_onlanguagechange(&mut self, value: jsbind::Any) {
        self.inner.set("onlanguagechange", value);
    }
}
impl WorkerGlobalScope {
    pub fn onoffline(&self) -> jsbind::Any {
        self.inner.get("onoffline").as_::<jsbind::Any>()
    }

    pub fn set_onoffline(&mut self, value: jsbind::Any) {
        self.inner.set("onoffline", value);
    }
}
impl WorkerGlobalScope {
    pub fn ononline(&self) -> jsbind::Any {
        self.inner.get("ononline").as_::<jsbind::Any>()
    }

    pub fn set_ononline(&mut self, value: jsbind::Any) {
        self.inner.set("ononline", value);
    }
}
impl WorkerGlobalScope {
    pub fn onrejectionhandled(&self) -> jsbind::Any {
        self.inner.get("onrejectionhandled").as_::<jsbind::Any>()
    }

    pub fn set_onrejectionhandled(&mut self, value: jsbind::Any) {
        self.inner.set("onrejectionhandled", value);
    }
}
impl WorkerGlobalScope {
    pub fn onunhandledrejection(&self) -> jsbind::Any {
        self.inner.get("onunhandledrejection").as_::<jsbind::Any>()
    }

    pub fn set_onunhandledrejection(&mut self, value: jsbind::Any) {
        self.inner.set("onunhandledrejection", value);
    }
}
impl WorkerGlobalScope {
    pub fn fonts(&self) -> FontFaceSet {
        self.inner.get("fonts").as_::<FontFaceSet>()
    }
}
impl WorkerGlobalScope {
    pub fn crypto(&self) -> Crypto {
        self.inner.get("crypto").as_::<Crypto>()
    }
}

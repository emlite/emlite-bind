use super::*;

#[derive(Clone, Debug)]
pub struct RegistrationOptions {
    inner: emlite::Val,
}
impl FromVal for RegistrationOptions {
    fn from_val(v: &emlite::Val) -> Self {
        RegistrationOptions { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for RegistrationOptions {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for RegistrationOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<RegistrationOptions> for emlite::Val {
    fn from(s: RegistrationOptions) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl RegistrationOptions {
    pub fn scope(&self) -> jsbind::USVString {
        self.inner.get("scope").as_::<jsbind::USVString>()
    }

    pub fn set_scope(&mut self, value: jsbind::USVString) {
        self.inner.set("scope", value);
    }
}
impl RegistrationOptions {
    pub fn type_(&self) -> WorkerType {
        self.inner.get("type").as_::<WorkerType>()
    }

    pub fn set_type_(&mut self, value: WorkerType) {
        self.inner.set("type", value);
    }
}
impl RegistrationOptions {
    pub fn update_via_cache(&self) -> ServiceWorkerUpdateViaCache {
        self.inner
            .get("updateViaCache")
            .as_::<ServiceWorkerUpdateViaCache>()
    }

    pub fn set_update_via_cache(&mut self, value: ServiceWorkerUpdateViaCache) {
        self.inner.set("updateViaCache", value);
    }
}
#[derive(Clone, Debug)]
pub struct ServiceWorkerContainer {
    inner: EventTarget,
}
impl FromVal for ServiceWorkerContainer {
    fn from_val(v: &emlite::Val) -> Self {
        ServiceWorkerContainer {
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
impl std::ops::Deref for ServiceWorkerContainer {
    type Target = EventTarget;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for ServiceWorkerContainer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<ServiceWorkerContainer> for emlite::Val {
    fn from(s: ServiceWorkerContainer) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl ServiceWorkerContainer {
    pub fn controller(&self) -> ServiceWorker {
        self.inner.get("controller").as_::<ServiceWorker>()
    }
}
impl ServiceWorkerContainer {
    pub fn ready(&self) -> jsbind::Promise {
        self.inner.get("ready").as_::<jsbind::Promise>()
    }
}
impl ServiceWorkerContainer {
    pub fn register0(&self, script_url: jsbind::Any) -> jsbind::Promise {
        self.inner
            .call("register", &[script_url.into()])
            .as_::<jsbind::Promise>()
    }

    pub fn register1(
        &self,
        script_url: jsbind::Any,
        options: RegistrationOptions,
    ) -> jsbind::Promise {
        self.inner
            .call("register", &[script_url.into(), options.into()])
            .as_::<jsbind::Promise>()
    }
}
impl ServiceWorkerContainer {
    pub fn get_registration0(&self) -> jsbind::Promise {
        self.inner
            .call("getRegistration", &[])
            .as_::<jsbind::Promise>()
    }

    pub fn get_registration1(&self, client_url: jsbind::USVString) -> jsbind::Promise {
        self.inner
            .call("getRegistration", &[client_url.into()])
            .as_::<jsbind::Promise>()
    }
}
impl ServiceWorkerContainer {
    pub fn get_registrations(&self) -> jsbind::Promise {
        self.inner
            .call("getRegistrations", &[])
            .as_::<jsbind::Promise>()
    }
}
impl ServiceWorkerContainer {
    pub fn start_messages(&self) -> jsbind::Undefined {
        self.inner
            .call("startMessages", &[])
            .as_::<jsbind::Undefined>()
    }
}
impl ServiceWorkerContainer {
    pub fn oncontrollerchange(&self) -> jsbind::Any {
        self.inner.get("oncontrollerchange").as_::<jsbind::Any>()
    }

    pub fn set_oncontrollerchange(&mut self, value: jsbind::Any) {
        self.inner.set("oncontrollerchange", value);
    }
}
impl ServiceWorkerContainer {
    pub fn onmessage(&self) -> jsbind::Any {
        self.inner.get("onmessage").as_::<jsbind::Any>()
    }

    pub fn set_onmessage(&mut self, value: jsbind::Any) {
        self.inner.set("onmessage", value);
    }
}
impl ServiceWorkerContainer {
    pub fn onmessageerror(&self) -> jsbind::Any {
        self.inner.get("onmessageerror").as_::<jsbind::Any>()
    }

    pub fn set_onmessageerror(&mut self, value: jsbind::Any) {
        self.inner.set("onmessageerror", value);
    }
}

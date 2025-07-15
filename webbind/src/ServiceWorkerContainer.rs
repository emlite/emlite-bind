use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
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
impl core::ops::Deref for RegistrationOptions {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for RegistrationOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for RegistrationOptions {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for RegistrationOptions {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<RegistrationOptions> for emlite::Val {
    fn from(s: RegistrationOptions) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&RegistrationOptions> for emlite::Val {
    fn from(s: &RegistrationOptions) -> emlite::Val {
        s.inner.clone()
    }
}

impl RegistrationOptions {
    pub fn scope(&self) -> String {
        self.inner.get("scope").as_::<String>()
    }

    pub fn set_scope(&mut self, value: &str) {
        self.inner.set("scope", value);
    }
}
impl RegistrationOptions {
    pub fn type_(&self) -> WorkerType {
        self.inner.get("type").as_::<WorkerType>()
    }

    pub fn set_type_(&mut self, value: &WorkerType) {
        self.inner.set("type", value);
    }
}
impl RegistrationOptions {
    pub fn update_via_cache(&self) -> ServiceWorkerUpdateViaCache {
        self.inner
            .get("updateViaCache")
            .as_::<ServiceWorkerUpdateViaCache>()
    }

    pub fn set_update_via_cache(&mut self, value: &ServiceWorkerUpdateViaCache) {
        self.inner.set("updateViaCache", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
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
impl core::ops::Deref for ServiceWorkerContainer {
    type Target = EventTarget;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for ServiceWorkerContainer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for ServiceWorkerContainer {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for ServiceWorkerContainer {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<ServiceWorkerContainer> for emlite::Val {
    fn from(s: ServiceWorkerContainer) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&ServiceWorkerContainer> for emlite::Val {
    fn from(s: &ServiceWorkerContainer) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(ServiceWorkerContainer);

impl ServiceWorkerContainer {
    pub fn controller(&self) -> ServiceWorker {
        self.inner.get("controller").as_::<ServiceWorker>()
    }
}
impl ServiceWorkerContainer {
    pub fn ready(&self) -> Promise {
        self.inner.get("ready").as_::<Promise>()
    }
}
impl ServiceWorkerContainer {
    pub fn register0(&self, script_url: &Any) -> Promise {
        self.inner
            .call("register", &[script_url.into()])
            .as_::<Promise>()
    }

    pub fn register1(&self, script_url: &Any, options: &RegistrationOptions) -> Promise {
        self.inner
            .call("register", &[script_url.into(), options.into()])
            .as_::<Promise>()
    }
}
impl ServiceWorkerContainer {
    pub fn get_registration0(&self) -> Promise {
        self.inner.call("getRegistration", &[]).as_::<Promise>()
    }

    pub fn get_registration1(&self, client_url: &str) -> Promise {
        self.inner
            .call("getRegistration", &[client_url.into()])
            .as_::<Promise>()
    }
}
impl ServiceWorkerContainer {
    pub fn get_registrations(&self) -> Promise {
        self.inner.call("getRegistrations", &[]).as_::<Promise>()
    }
}
impl ServiceWorkerContainer {
    pub fn start_messages(&self) -> Undefined {
        self.inner.call("startMessages", &[]).as_::<Undefined>()
    }
}
impl ServiceWorkerContainer {
    pub fn oncontrollerchange(&self) -> Any {
        self.inner.get("oncontrollerchange").as_::<Any>()
    }

    pub fn set_oncontrollerchange(&mut self, value: &Any) {
        self.inner.set("oncontrollerchange", value);
    }
}
impl ServiceWorkerContainer {
    pub fn onmessage(&self) -> Any {
        self.inner.get("onmessage").as_::<Any>()
    }

    pub fn set_onmessage(&mut self, value: &Any) {
        self.inner.set("onmessage", value);
    }
}
impl ServiceWorkerContainer {
    pub fn onmessageerror(&self) -> Any {
        self.inner.get("onmessageerror").as_::<Any>()
    }

    pub fn set_onmessageerror(&mut self, value: &Any) {
        self.inner.set("onmessageerror", value);
    }
}

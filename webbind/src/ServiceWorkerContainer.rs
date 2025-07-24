use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct RegistrationOptions {
    inner: Any,
}
impl FromVal for RegistrationOptions {
    fn from_val(v: &Any) -> Self {
        RegistrationOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for RegistrationOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for RegistrationOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for RegistrationOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for RegistrationOptions {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<RegistrationOptions> for Any {
    fn from(s: RegistrationOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&RegistrationOptions> for Any {
    fn from(s: &RegistrationOptions) -> Any {
        s.inner.clone()
    }
}

impl RegistrationOptions {
    pub fn scope(&self) -> USVString {
        self.inner.get("scope").as_::<USVString>()
    }

    pub fn set_scope(&mut self, value: &USVString) {
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
/// The ServiceWorkerContainer class.
/// [`ServiceWorkerContainer`](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerContainer)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct ServiceWorkerContainer {
    inner: EventTarget,
}
impl FromVal for ServiceWorkerContainer {
    fn from_val(v: &Any) -> Self {
        ServiceWorkerContainer {
            inner: EventTarget::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
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
impl AsRef<Any> for ServiceWorkerContainer {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for ServiceWorkerContainer {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<ServiceWorkerContainer> for Any {
    fn from(s: ServiceWorkerContainer) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&ServiceWorkerContainer> for Any {
    fn from(s: &ServiceWorkerContainer) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(ServiceWorkerContainer);

impl ServiceWorkerContainer {
    /// Getter of the `controller` attribute.
    /// [`ServiceWorkerContainer.controller`](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerContainer/controller)
    pub fn controller(&self) -> ServiceWorker {
        self.inner.get("controller").as_::<ServiceWorker>()
    }
}
impl ServiceWorkerContainer {
    /// Getter of the `ready` attribute.
    /// [`ServiceWorkerContainer.ready`](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerContainer/ready)
    pub fn ready(&self) -> Promise<ServiceWorkerRegistration> {
        self.inner
            .get("ready")
            .as_::<Promise<ServiceWorkerRegistration>>()
    }
}
impl ServiceWorkerContainer {
    /// The register method.
    /// [`ServiceWorkerContainer.register`](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerContainer/register)
    pub fn register0(&self, script_url: &Any) -> Promise<ServiceWorkerRegistration> {
        self.inner
            .call("register", &[script_url.into()])
            .as_::<Promise<ServiceWorkerRegistration>>()
    }
    /// The register method.
    /// [`ServiceWorkerContainer.register`](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerContainer/register)
    pub fn register1(
        &self,
        script_url: &Any,
        options: &RegistrationOptions,
    ) -> Promise<ServiceWorkerRegistration> {
        self.inner
            .call("register", &[script_url.into(), options.into()])
            .as_::<Promise<ServiceWorkerRegistration>>()
    }
}
impl ServiceWorkerContainer {
    /// The getRegistration method.
    /// [`ServiceWorkerContainer.getRegistration`](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerContainer/getRegistration)
    pub fn get_registration0(&self) -> Promise<Any> {
        self.inner
            .call("getRegistration", &[])
            .as_::<Promise<Any>>()
    }
    /// The getRegistration method.
    /// [`ServiceWorkerContainer.getRegistration`](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerContainer/getRegistration)
    pub fn get_registration1(&self, client_url: &USVString) -> Promise<Any> {
        self.inner
            .call("getRegistration", &[client_url.into()])
            .as_::<Promise<Any>>()
    }
}
impl ServiceWorkerContainer {
    /// The getRegistrations method.
    /// [`ServiceWorkerContainer.getRegistrations`](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerContainer/getRegistrations)
    pub fn get_registrations(&self) -> Promise<FrozenArray<ServiceWorkerRegistration>> {
        self.inner
            .call("getRegistrations", &[])
            .as_::<Promise<FrozenArray<ServiceWorkerRegistration>>>()
    }
}
impl ServiceWorkerContainer {
    /// The startMessages method.
    /// [`ServiceWorkerContainer.startMessages`](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerContainer/startMessages)
    pub fn start_messages(&self) -> Undefined {
        self.inner.call("startMessages", &[]).as_::<Undefined>()
    }
}
impl ServiceWorkerContainer {
    /// Getter of the `oncontrollerchange` attribute.
    /// [`ServiceWorkerContainer.oncontrollerchange`](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerContainer/oncontrollerchange)
    pub fn oncontrollerchange(&self) -> Any {
        self.inner.get("oncontrollerchange").as_::<Any>()
    }

    /// Setter of the `oncontrollerchange` attribute.
    /// [`ServiceWorkerContainer.oncontrollerchange`](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerContainer/oncontrollerchange)
    pub fn set_oncontrollerchange(&mut self, value: &Any) {
        self.inner.set("oncontrollerchange", value);
    }
}
impl ServiceWorkerContainer {
    /// Getter of the `onmessage` attribute.
    /// [`ServiceWorkerContainer.onmessage`](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerContainer/onmessage)
    pub fn onmessage(&self) -> Any {
        self.inner.get("onmessage").as_::<Any>()
    }

    /// Setter of the `onmessage` attribute.
    /// [`ServiceWorkerContainer.onmessage`](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerContainer/onmessage)
    pub fn set_onmessage(&mut self, value: &Any) {
        self.inner.set("onmessage", value);
    }
}
impl ServiceWorkerContainer {
    /// Getter of the `onmessageerror` attribute.
    /// [`ServiceWorkerContainer.onmessageerror`](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerContainer/onmessageerror)
    pub fn onmessageerror(&self) -> Any {
        self.inner.get("onmessageerror").as_::<Any>()
    }

    /// Setter of the `onmessageerror` attribute.
    /// [`ServiceWorkerContainer.onmessageerror`](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerContainer/onmessageerror)
    pub fn set_onmessageerror(&mut self, value: &Any) {
        self.inner.set("onmessageerror", value);
    }
}

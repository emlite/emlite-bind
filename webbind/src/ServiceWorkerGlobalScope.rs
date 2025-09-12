use super::*;

/// The ServiceWorkerGlobalScope class.
/// [`ServiceWorkerGlobalScope`](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerGlobalScope)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct ServiceWorkerGlobalScope {
    inner: WorkerGlobalScope,
}

impl FromVal for ServiceWorkerGlobalScope {
    fn from_val(v: &Any) -> Self {
        ServiceWorkerGlobalScope {
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

impl core::ops::Deref for ServiceWorkerGlobalScope {
    type Target = WorkerGlobalScope;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for ServiceWorkerGlobalScope {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for ServiceWorkerGlobalScope {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for ServiceWorkerGlobalScope {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<ServiceWorkerGlobalScope> for Any {
    fn from(s: ServiceWorkerGlobalScope) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&ServiceWorkerGlobalScope> for Any {
    fn from(s: &ServiceWorkerGlobalScope) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(ServiceWorkerGlobalScope);

impl ServiceWorkerGlobalScope {
    /// Getter of the `clients` attribute.
    /// [`ServiceWorkerGlobalScope.clients`](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerGlobalScope/clients)
    pub fn clients(&self) -> Clients {
        self.inner.get("clients").as_::<Clients>()
    }
}
impl ServiceWorkerGlobalScope {
    /// Getter of the `registration` attribute.
    /// [`ServiceWorkerGlobalScope.registration`](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerGlobalScope/registration)
    pub fn registration(&self) -> ServiceWorkerRegistration {
        self.inner
            .get("registration")
            .as_::<ServiceWorkerRegistration>()
    }
}
impl ServiceWorkerGlobalScope {
    /// Getter of the `serviceWorker` attribute.
    /// [`ServiceWorkerGlobalScope.serviceWorker`](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerGlobalScope/serviceWorker)
    pub fn service_worker(&self) -> ServiceWorker {
        self.inner.get("serviceWorker").as_::<ServiceWorker>()
    }
}
impl ServiceWorkerGlobalScope {
    /// Getter of the `oninstall` attribute.
    /// [`ServiceWorkerGlobalScope.oninstall`](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerGlobalScope/oninstall)
    pub fn oninstall(&self) -> Any {
        self.inner.get("oninstall").as_::<Any>()
    }

    /// Setter of the `oninstall` attribute.
    /// [`ServiceWorkerGlobalScope.oninstall`](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerGlobalScope/oninstall)
    pub fn set_oninstall(&mut self, value: &Any) {
        self.inner.set("oninstall", value);
    }
}
impl ServiceWorkerGlobalScope {
    /// Getter of the `onactivate` attribute.
    /// [`ServiceWorkerGlobalScope.onactivate`](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerGlobalScope/onactivate)
    pub fn onactivate(&self) -> Any {
        self.inner.get("onactivate").as_::<Any>()
    }

    /// Setter of the `onactivate` attribute.
    /// [`ServiceWorkerGlobalScope.onactivate`](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerGlobalScope/onactivate)
    pub fn set_onactivate(&mut self, value: &Any) {
        self.inner.set("onactivate", value);
    }
}
impl ServiceWorkerGlobalScope {
    /// Getter of the `onfetch` attribute.
    /// [`ServiceWorkerGlobalScope.onfetch`](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerGlobalScope/onfetch)
    pub fn onfetch(&self) -> Any {
        self.inner.get("onfetch").as_::<Any>()
    }

    /// Setter of the `onfetch` attribute.
    /// [`ServiceWorkerGlobalScope.onfetch`](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerGlobalScope/onfetch)
    pub fn set_onfetch(&mut self, value: &Any) {
        self.inner.set("onfetch", value);
    }
}
impl ServiceWorkerGlobalScope {
    /// Getter of the `onmessage` attribute.
    /// [`ServiceWorkerGlobalScope.onmessage`](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerGlobalScope/onmessage)
    pub fn onmessage(&self) -> Any {
        self.inner.get("onmessage").as_::<Any>()
    }

    /// Setter of the `onmessage` attribute.
    /// [`ServiceWorkerGlobalScope.onmessage`](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerGlobalScope/onmessage)
    pub fn set_onmessage(&mut self, value: &Any) {
        self.inner.set("onmessage", value);
    }
}
impl ServiceWorkerGlobalScope {
    /// Getter of the `onmessageerror` attribute.
    /// [`ServiceWorkerGlobalScope.onmessageerror`](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerGlobalScope/onmessageerror)
    pub fn onmessageerror(&self) -> Any {
        self.inner.get("onmessageerror").as_::<Any>()
    }

    /// Setter of the `onmessageerror` attribute.
    /// [`ServiceWorkerGlobalScope.onmessageerror`](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerGlobalScope/onmessageerror)
    pub fn set_onmessageerror(&mut self, value: &Any) {
        self.inner.set("onmessageerror", value);
    }
}
impl ServiceWorkerGlobalScope {
    /// Getter of the `onbackgroundfetchsuccess` attribute.
    /// [`ServiceWorkerGlobalScope.onbackgroundfetchsuccess`](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerGlobalScope/onbackgroundfetchsuccess)
    pub fn onbackgroundfetchsuccess(&self) -> Any {
        self.inner.get("onbackgroundfetchsuccess").as_::<Any>()
    }

    /// Setter of the `onbackgroundfetchsuccess` attribute.
    /// [`ServiceWorkerGlobalScope.onbackgroundfetchsuccess`](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerGlobalScope/onbackgroundfetchsuccess)
    pub fn set_onbackgroundfetchsuccess(&mut self, value: &Any) {
        self.inner.set("onbackgroundfetchsuccess", value);
    }
}
impl ServiceWorkerGlobalScope {
    /// Getter of the `onbackgroundfetchfail` attribute.
    /// [`ServiceWorkerGlobalScope.onbackgroundfetchfail`](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerGlobalScope/onbackgroundfetchfail)
    pub fn onbackgroundfetchfail(&self) -> Any {
        self.inner.get("onbackgroundfetchfail").as_::<Any>()
    }

    /// Setter of the `onbackgroundfetchfail` attribute.
    /// [`ServiceWorkerGlobalScope.onbackgroundfetchfail`](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerGlobalScope/onbackgroundfetchfail)
    pub fn set_onbackgroundfetchfail(&mut self, value: &Any) {
        self.inner.set("onbackgroundfetchfail", value);
    }
}
impl ServiceWorkerGlobalScope {
    /// Getter of the `onbackgroundfetchabort` attribute.
    /// [`ServiceWorkerGlobalScope.onbackgroundfetchabort`](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerGlobalScope/onbackgroundfetchabort)
    pub fn onbackgroundfetchabort(&self) -> Any {
        self.inner.get("onbackgroundfetchabort").as_::<Any>()
    }

    /// Setter of the `onbackgroundfetchabort` attribute.
    /// [`ServiceWorkerGlobalScope.onbackgroundfetchabort`](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerGlobalScope/onbackgroundfetchabort)
    pub fn set_onbackgroundfetchabort(&mut self, value: &Any) {
        self.inner.set("onbackgroundfetchabort", value);
    }
}
impl ServiceWorkerGlobalScope {
    /// Getter of the `onbackgroundfetchclick` attribute.
    /// [`ServiceWorkerGlobalScope.onbackgroundfetchclick`](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerGlobalScope/onbackgroundfetchclick)
    pub fn onbackgroundfetchclick(&self) -> Any {
        self.inner.get("onbackgroundfetchclick").as_::<Any>()
    }

    /// Setter of the `onbackgroundfetchclick` attribute.
    /// [`ServiceWorkerGlobalScope.onbackgroundfetchclick`](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerGlobalScope/onbackgroundfetchclick)
    pub fn set_onbackgroundfetchclick(&mut self, value: &Any) {
        self.inner.set("onbackgroundfetchclick", value);
    }
}
impl ServiceWorkerGlobalScope {
    /// Getter of the `onsync` attribute.
    /// [`ServiceWorkerGlobalScope.onsync`](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerGlobalScope/onsync)
    pub fn onsync(&self) -> Any {
        self.inner.get("onsync").as_::<Any>()
    }

    /// Setter of the `onsync` attribute.
    /// [`ServiceWorkerGlobalScope.onsync`](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerGlobalScope/onsync)
    pub fn set_onsync(&mut self, value: &Any) {
        self.inner.set("onsync", value);
    }
}
impl ServiceWorkerGlobalScope {
    /// Getter of the `oncontentdelete` attribute.
    /// [`ServiceWorkerGlobalScope.oncontentdelete`](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerGlobalScope/oncontentdelete)
    pub fn oncontentdelete(&self) -> Any {
        self.inner.get("oncontentdelete").as_::<Any>()
    }

    /// Setter of the `oncontentdelete` attribute.
    /// [`ServiceWorkerGlobalScope.oncontentdelete`](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerGlobalScope/oncontentdelete)
    pub fn set_oncontentdelete(&mut self, value: &Any) {
        self.inner.set("oncontentdelete", value);
    }
}
impl ServiceWorkerGlobalScope {
    /// Getter of the `cookieStore` attribute.
    /// [`ServiceWorkerGlobalScope.cookieStore`](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerGlobalScope/cookieStore)
    pub fn cookie_store(&self) -> CookieStore {
        self.inner.get("cookieStore").as_::<CookieStore>()
    }
}
impl ServiceWorkerGlobalScope {
    /// Getter of the `oncookiechange` attribute.
    /// [`ServiceWorkerGlobalScope.oncookiechange`](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerGlobalScope/oncookiechange)
    pub fn oncookiechange(&self) -> Any {
        self.inner.get("oncookiechange").as_::<Any>()
    }

    /// Setter of the `oncookiechange` attribute.
    /// [`ServiceWorkerGlobalScope.oncookiechange`](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerGlobalScope/oncookiechange)
    pub fn set_oncookiechange(&mut self, value: &Any) {
        self.inner.set("oncookiechange", value);
    }
}
impl ServiceWorkerGlobalScope {
    /// Getter of the `onnotificationclick` attribute.
    /// [`ServiceWorkerGlobalScope.onnotificationclick`](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerGlobalScope/onnotificationclick)
    pub fn onnotificationclick(&self) -> Any {
        self.inner.get("onnotificationclick").as_::<Any>()
    }

    /// Setter of the `onnotificationclick` attribute.
    /// [`ServiceWorkerGlobalScope.onnotificationclick`](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerGlobalScope/onnotificationclick)
    pub fn set_onnotificationclick(&mut self, value: &Any) {
        self.inner.set("onnotificationclick", value);
    }
}
impl ServiceWorkerGlobalScope {
    /// Getter of the `onnotificationclose` attribute.
    /// [`ServiceWorkerGlobalScope.onnotificationclose`](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerGlobalScope/onnotificationclose)
    pub fn onnotificationclose(&self) -> Any {
        self.inner.get("onnotificationclose").as_::<Any>()
    }

    /// Setter of the `onnotificationclose` attribute.
    /// [`ServiceWorkerGlobalScope.onnotificationclose`](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerGlobalScope/onnotificationclose)
    pub fn set_onnotificationclose(&mut self, value: &Any) {
        self.inner.set("onnotificationclose", value);
    }
}
impl ServiceWorkerGlobalScope {
    /// Getter of the `oncanmakepayment` attribute.
    /// [`ServiceWorkerGlobalScope.oncanmakepayment`](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerGlobalScope/oncanmakepayment)
    pub fn oncanmakepayment(&self) -> Any {
        self.inner.get("oncanmakepayment").as_::<Any>()
    }

    /// Setter of the `oncanmakepayment` attribute.
    /// [`ServiceWorkerGlobalScope.oncanmakepayment`](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerGlobalScope/oncanmakepayment)
    pub fn set_oncanmakepayment(&mut self, value: &Any) {
        self.inner.set("oncanmakepayment", value);
    }
}
impl ServiceWorkerGlobalScope {
    /// Getter of the `onpaymentrequest` attribute.
    /// [`ServiceWorkerGlobalScope.onpaymentrequest`](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerGlobalScope/onpaymentrequest)
    pub fn onpaymentrequest(&self) -> Any {
        self.inner.get("onpaymentrequest").as_::<Any>()
    }

    /// Setter of the `onpaymentrequest` attribute.
    /// [`ServiceWorkerGlobalScope.onpaymentrequest`](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerGlobalScope/onpaymentrequest)
    pub fn set_onpaymentrequest(&mut self, value: &Any) {
        self.inner.set("onpaymentrequest", value);
    }
}
impl ServiceWorkerGlobalScope {
    /// Getter of the `onperiodicsync` attribute.
    /// [`ServiceWorkerGlobalScope.onperiodicsync`](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerGlobalScope/onperiodicsync)
    pub fn onperiodicsync(&self) -> Any {
        self.inner.get("onperiodicsync").as_::<Any>()
    }

    /// Setter of the `onperiodicsync` attribute.
    /// [`ServiceWorkerGlobalScope.onperiodicsync`](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerGlobalScope/onperiodicsync)
    pub fn set_onperiodicsync(&mut self, value: &Any) {
        self.inner.set("onperiodicsync", value);
    }
}
impl ServiceWorkerGlobalScope {
    /// Getter of the `onpush` attribute.
    /// [`ServiceWorkerGlobalScope.onpush`](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerGlobalScope/onpush)
    pub fn onpush(&self) -> Any {
        self.inner.get("onpush").as_::<Any>()
    }

    /// Setter of the `onpush` attribute.
    /// [`ServiceWorkerGlobalScope.onpush`](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerGlobalScope/onpush)
    pub fn set_onpush(&mut self, value: &Any) {
        self.inner.set("onpush", value);
    }
}
impl ServiceWorkerGlobalScope {
    /// Getter of the `onpushsubscriptionchange` attribute.
    /// [`ServiceWorkerGlobalScope.onpushsubscriptionchange`](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerGlobalScope/onpushsubscriptionchange)
    pub fn onpushsubscriptionchange(&self) -> Any {
        self.inner.get("onpushsubscriptionchange").as_::<Any>()
    }

    /// Setter of the `onpushsubscriptionchange` attribute.
    /// [`ServiceWorkerGlobalScope.onpushsubscriptionchange`](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerGlobalScope/onpushsubscriptionchange)
    pub fn set_onpushsubscriptionchange(&mut self, value: &Any) {
        self.inner.set("onpushsubscriptionchange", value);
    }
}
impl ServiceWorkerGlobalScope {
    /// The skipWaiting method.
    /// [`ServiceWorkerGlobalScope.skipWaiting`](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerGlobalScope/skipWaiting)
    pub fn skip_waiting(&self) -> Promise<Undefined> {
        self.inner
            .call("skipWaiting", &[])
            .as_::<Promise<Undefined>>()
    }
}

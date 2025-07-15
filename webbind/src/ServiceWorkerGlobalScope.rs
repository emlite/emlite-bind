use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct ServiceWorkerGlobalScope {
    inner: WorkerGlobalScope,
}
impl FromVal for ServiceWorkerGlobalScope {
    fn from_val(v: &emlite::Val) -> Self {
        ServiceWorkerGlobalScope {
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
impl AsRef<emlite::Val> for ServiceWorkerGlobalScope {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for ServiceWorkerGlobalScope {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<ServiceWorkerGlobalScope> for emlite::Val {
    fn from(s: ServiceWorkerGlobalScope) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(ServiceWorkerGlobalScope);

impl ServiceWorkerGlobalScope {
    pub fn clients(&self) -> Clients {
        self.inner.get("clients").as_::<Clients>()
    }
}
impl ServiceWorkerGlobalScope {
    pub fn registration(&self) -> ServiceWorkerRegistration {
        self.inner
            .get("registration")
            .as_::<ServiceWorkerRegistration>()
    }
}
impl ServiceWorkerGlobalScope {
    pub fn service_worker(&self) -> ServiceWorker {
        self.inner.get("serviceWorker").as_::<ServiceWorker>()
    }
}
impl ServiceWorkerGlobalScope {
    pub fn skip_waiting(&self) -> Promise {
        self.inner.call("skipWaiting", &[]).as_::<Promise>()
    }
}
impl ServiceWorkerGlobalScope {
    pub fn oninstall(&self) -> Any {
        self.inner.get("oninstall").as_::<Any>()
    }

    pub fn set_oninstall(&mut self, value: Any) {
        self.inner.set("oninstall", value);
    }
}
impl ServiceWorkerGlobalScope {
    pub fn onactivate(&self) -> Any {
        self.inner.get("onactivate").as_::<Any>()
    }

    pub fn set_onactivate(&mut self, value: Any) {
        self.inner.set("onactivate", value);
    }
}
impl ServiceWorkerGlobalScope {
    pub fn onfetch(&self) -> Any {
        self.inner.get("onfetch").as_::<Any>()
    }

    pub fn set_onfetch(&mut self, value: Any) {
        self.inner.set("onfetch", value);
    }
}
impl ServiceWorkerGlobalScope {
    pub fn onmessage(&self) -> Any {
        self.inner.get("onmessage").as_::<Any>()
    }

    pub fn set_onmessage(&mut self, value: Any) {
        self.inner.set("onmessage", value);
    }
}
impl ServiceWorkerGlobalScope {
    pub fn onmessageerror(&self) -> Any {
        self.inner.get("onmessageerror").as_::<Any>()
    }

    pub fn set_onmessageerror(&mut self, value: Any) {
        self.inner.set("onmessageerror", value);
    }
}
impl ServiceWorkerGlobalScope {
    pub fn onbackgroundfetchsuccess(&self) -> Any {
        self.inner.get("onbackgroundfetchsuccess").as_::<Any>()
    }

    pub fn set_onbackgroundfetchsuccess(&mut self, value: Any) {
        self.inner.set("onbackgroundfetchsuccess", value);
    }
}
impl ServiceWorkerGlobalScope {
    pub fn onbackgroundfetchfail(&self) -> Any {
        self.inner.get("onbackgroundfetchfail").as_::<Any>()
    }

    pub fn set_onbackgroundfetchfail(&mut self, value: Any) {
        self.inner.set("onbackgroundfetchfail", value);
    }
}
impl ServiceWorkerGlobalScope {
    pub fn onbackgroundfetchabort(&self) -> Any {
        self.inner.get("onbackgroundfetchabort").as_::<Any>()
    }

    pub fn set_onbackgroundfetchabort(&mut self, value: Any) {
        self.inner.set("onbackgroundfetchabort", value);
    }
}
impl ServiceWorkerGlobalScope {
    pub fn onbackgroundfetchclick(&self) -> Any {
        self.inner.get("onbackgroundfetchclick").as_::<Any>()
    }

    pub fn set_onbackgroundfetchclick(&mut self, value: Any) {
        self.inner.set("onbackgroundfetchclick", value);
    }
}
impl ServiceWorkerGlobalScope {
    pub fn onsync(&self) -> Any {
        self.inner.get("onsync").as_::<Any>()
    }

    pub fn set_onsync(&mut self, value: Any) {
        self.inner.set("onsync", value);
    }
}
impl ServiceWorkerGlobalScope {
    pub fn oncontentdelete(&self) -> Any {
        self.inner.get("oncontentdelete").as_::<Any>()
    }

    pub fn set_oncontentdelete(&mut self, value: Any) {
        self.inner.set("oncontentdelete", value);
    }
}
impl ServiceWorkerGlobalScope {
    pub fn cookie_store(&self) -> CookieStore {
        self.inner.get("cookieStore").as_::<CookieStore>()
    }
}
impl ServiceWorkerGlobalScope {
    pub fn oncookiechange(&self) -> Any {
        self.inner.get("oncookiechange").as_::<Any>()
    }

    pub fn set_oncookiechange(&mut self, value: Any) {
        self.inner.set("oncookiechange", value);
    }
}
impl ServiceWorkerGlobalScope {
    pub fn onnotificationclick(&self) -> Any {
        self.inner.get("onnotificationclick").as_::<Any>()
    }

    pub fn set_onnotificationclick(&mut self, value: Any) {
        self.inner.set("onnotificationclick", value);
    }
}
impl ServiceWorkerGlobalScope {
    pub fn onnotificationclose(&self) -> Any {
        self.inner.get("onnotificationclose").as_::<Any>()
    }

    pub fn set_onnotificationclose(&mut self, value: Any) {
        self.inner.set("onnotificationclose", value);
    }
}
impl ServiceWorkerGlobalScope {
    pub fn oncanmakepayment(&self) -> Any {
        self.inner.get("oncanmakepayment").as_::<Any>()
    }

    pub fn set_oncanmakepayment(&mut self, value: Any) {
        self.inner.set("oncanmakepayment", value);
    }
}
impl ServiceWorkerGlobalScope {
    pub fn onpaymentrequest(&self) -> Any {
        self.inner.get("onpaymentrequest").as_::<Any>()
    }

    pub fn set_onpaymentrequest(&mut self, value: Any) {
        self.inner.set("onpaymentrequest", value);
    }
}
impl ServiceWorkerGlobalScope {
    pub fn onperiodicsync(&self) -> Any {
        self.inner.get("onperiodicsync").as_::<Any>()
    }

    pub fn set_onperiodicsync(&mut self, value: Any) {
        self.inner.set("onperiodicsync", value);
    }
}
impl ServiceWorkerGlobalScope {
    pub fn onpush(&self) -> Any {
        self.inner.get("onpush").as_::<Any>()
    }

    pub fn set_onpush(&mut self, value: Any) {
        self.inner.set("onpush", value);
    }
}
impl ServiceWorkerGlobalScope {
    pub fn onpushsubscriptionchange(&self) -> Any {
        self.inner.get("onpushsubscriptionchange").as_::<Any>()
    }

    pub fn set_onpushsubscriptionchange(&mut self, value: Any) {
        self.inner.set("onpushsubscriptionchange", value);
    }
}

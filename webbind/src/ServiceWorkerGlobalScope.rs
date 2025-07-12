use super::*;

#[derive(Clone, Debug)]
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
impl std::ops::Deref for ServiceWorkerGlobalScope {
    type Target = WorkerGlobalScope;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for ServiceWorkerGlobalScope {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<ServiceWorkerGlobalScope> for emlite::Val {
    fn from(s: ServiceWorkerGlobalScope) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

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
    pub fn skip_waiting(&self) -> jsbind::Promise {
        self.inner.call("skipWaiting", &[]).as_::<jsbind::Promise>()
    }
}
impl ServiceWorkerGlobalScope {
    pub fn oninstall(&self) -> jsbind::Any {
        self.inner.get("oninstall").as_::<jsbind::Any>()
    }

    pub fn set_oninstall(&mut self, value: jsbind::Any) {
        self.inner.set("oninstall", value);
    }
}
impl ServiceWorkerGlobalScope {
    pub fn onactivate(&self) -> jsbind::Any {
        self.inner.get("onactivate").as_::<jsbind::Any>()
    }

    pub fn set_onactivate(&mut self, value: jsbind::Any) {
        self.inner.set("onactivate", value);
    }
}
impl ServiceWorkerGlobalScope {
    pub fn onfetch(&self) -> jsbind::Any {
        self.inner.get("onfetch").as_::<jsbind::Any>()
    }

    pub fn set_onfetch(&mut self, value: jsbind::Any) {
        self.inner.set("onfetch", value);
    }
}
impl ServiceWorkerGlobalScope {
    pub fn onmessage(&self) -> jsbind::Any {
        self.inner.get("onmessage").as_::<jsbind::Any>()
    }

    pub fn set_onmessage(&mut self, value: jsbind::Any) {
        self.inner.set("onmessage", value);
    }
}
impl ServiceWorkerGlobalScope {
    pub fn onmessageerror(&self) -> jsbind::Any {
        self.inner.get("onmessageerror").as_::<jsbind::Any>()
    }

    pub fn set_onmessageerror(&mut self, value: jsbind::Any) {
        self.inner.set("onmessageerror", value);
    }
}
impl ServiceWorkerGlobalScope {
    pub fn onbackgroundfetchsuccess(&self) -> jsbind::Any {
        self.inner
            .get("onbackgroundfetchsuccess")
            .as_::<jsbind::Any>()
    }

    pub fn set_onbackgroundfetchsuccess(&mut self, value: jsbind::Any) {
        self.inner.set("onbackgroundfetchsuccess", value);
    }
}
impl ServiceWorkerGlobalScope {
    pub fn onbackgroundfetchfail(&self) -> jsbind::Any {
        self.inner.get("onbackgroundfetchfail").as_::<jsbind::Any>()
    }

    pub fn set_onbackgroundfetchfail(&mut self, value: jsbind::Any) {
        self.inner.set("onbackgroundfetchfail", value);
    }
}
impl ServiceWorkerGlobalScope {
    pub fn onbackgroundfetchabort(&self) -> jsbind::Any {
        self.inner
            .get("onbackgroundfetchabort")
            .as_::<jsbind::Any>()
    }

    pub fn set_onbackgroundfetchabort(&mut self, value: jsbind::Any) {
        self.inner.set("onbackgroundfetchabort", value);
    }
}
impl ServiceWorkerGlobalScope {
    pub fn onbackgroundfetchclick(&self) -> jsbind::Any {
        self.inner
            .get("onbackgroundfetchclick")
            .as_::<jsbind::Any>()
    }

    pub fn set_onbackgroundfetchclick(&mut self, value: jsbind::Any) {
        self.inner.set("onbackgroundfetchclick", value);
    }
}
impl ServiceWorkerGlobalScope {
    pub fn onsync(&self) -> jsbind::Any {
        self.inner.get("onsync").as_::<jsbind::Any>()
    }

    pub fn set_onsync(&mut self, value: jsbind::Any) {
        self.inner.set("onsync", value);
    }
}
impl ServiceWorkerGlobalScope {
    pub fn oncontentdelete(&self) -> jsbind::Any {
        self.inner.get("oncontentdelete").as_::<jsbind::Any>()
    }

    pub fn set_oncontentdelete(&mut self, value: jsbind::Any) {
        self.inner.set("oncontentdelete", value);
    }
}
impl ServiceWorkerGlobalScope {
    pub fn cookie_store(&self) -> CookieStore {
        self.inner.get("cookieStore").as_::<CookieStore>()
    }
}
impl ServiceWorkerGlobalScope {
    pub fn oncookiechange(&self) -> jsbind::Any {
        self.inner.get("oncookiechange").as_::<jsbind::Any>()
    }

    pub fn set_oncookiechange(&mut self, value: jsbind::Any) {
        self.inner.set("oncookiechange", value);
    }
}
impl ServiceWorkerGlobalScope {
    pub fn onnotificationclick(&self) -> jsbind::Any {
        self.inner.get("onnotificationclick").as_::<jsbind::Any>()
    }

    pub fn set_onnotificationclick(&mut self, value: jsbind::Any) {
        self.inner.set("onnotificationclick", value);
    }
}
impl ServiceWorkerGlobalScope {
    pub fn onnotificationclose(&self) -> jsbind::Any {
        self.inner.get("onnotificationclose").as_::<jsbind::Any>()
    }

    pub fn set_onnotificationclose(&mut self, value: jsbind::Any) {
        self.inner.set("onnotificationclose", value);
    }
}
impl ServiceWorkerGlobalScope {
    pub fn oncanmakepayment(&self) -> jsbind::Any {
        self.inner.get("oncanmakepayment").as_::<jsbind::Any>()
    }

    pub fn set_oncanmakepayment(&mut self, value: jsbind::Any) {
        self.inner.set("oncanmakepayment", value);
    }
}
impl ServiceWorkerGlobalScope {
    pub fn onpaymentrequest(&self) -> jsbind::Any {
        self.inner.get("onpaymentrequest").as_::<jsbind::Any>()
    }

    pub fn set_onpaymentrequest(&mut self, value: jsbind::Any) {
        self.inner.set("onpaymentrequest", value);
    }
}
impl ServiceWorkerGlobalScope {
    pub fn onperiodicsync(&self) -> jsbind::Any {
        self.inner.get("onperiodicsync").as_::<jsbind::Any>()
    }

    pub fn set_onperiodicsync(&mut self, value: jsbind::Any) {
        self.inner.set("onperiodicsync", value);
    }
}
impl ServiceWorkerGlobalScope {
    pub fn onpush(&self) -> jsbind::Any {
        self.inner.get("onpush").as_::<jsbind::Any>()
    }

    pub fn set_onpush(&mut self, value: jsbind::Any) {
        self.inner.set("onpush", value);
    }
}
impl ServiceWorkerGlobalScope {
    pub fn onpushsubscriptionchange(&self) -> jsbind::Any {
        self.inner
            .get("onpushsubscriptionchange")
            .as_::<jsbind::Any>()
    }

    pub fn set_onpushsubscriptionchange(&mut self, value: jsbind::Any) {
        self.inner.set("onpushsubscriptionchange", value);
    }
}

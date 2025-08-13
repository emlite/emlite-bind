use super::*;




/// The ServiceWorkerRegistration class.
/// [`ServiceWorkerRegistration`](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerRegistration)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct ServiceWorkerRegistration {
    inner: EventTarget,
}

impl FromVal for ServiceWorkerRegistration {
    fn from_val(v: &Any) -> Self {
        ServiceWorkerRegistration { inner: EventTarget::from_val(v) }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for ServiceWorkerRegistration {
    type Target = EventTarget;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for ServiceWorkerRegistration {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for ServiceWorkerRegistration {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for ServiceWorkerRegistration {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<ServiceWorkerRegistration> for Any {
    fn from(s: ServiceWorkerRegistration) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&ServiceWorkerRegistration> for Any {
    fn from(s: &ServiceWorkerRegistration) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(ServiceWorkerRegistration);


impl ServiceWorkerRegistration {
    /// Getter of the `installing` attribute.
    /// [`ServiceWorkerRegistration.installing`](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerRegistration/installing)
    pub fn installing(&self) -> ServiceWorker {
        self.inner.get("installing").as_::<ServiceWorker>()
    }

}
impl ServiceWorkerRegistration {
    /// Getter of the `waiting` attribute.
    /// [`ServiceWorkerRegistration.waiting`](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerRegistration/waiting)
    pub fn waiting(&self) -> ServiceWorker {
        self.inner.get("waiting").as_::<ServiceWorker>()
    }

}
impl ServiceWorkerRegistration {
    /// Getter of the `active` attribute.
    /// [`ServiceWorkerRegistration.active`](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerRegistration/active)
    pub fn active(&self) -> ServiceWorker {
        self.inner.get("active").as_::<ServiceWorker>()
    }

}
impl ServiceWorkerRegistration {
    /// Getter of the `navigationPreload` attribute.
    /// [`ServiceWorkerRegistration.navigationPreload`](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerRegistration/navigationPreload)
    pub fn navigation_preload(&self) -> NavigationPreloadManager {
        self.inner.get("navigationPreload").as_::<NavigationPreloadManager>()
    }

}
impl ServiceWorkerRegistration {
    /// Getter of the `scope` attribute.
    /// [`ServiceWorkerRegistration.scope`](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerRegistration/scope)
    pub fn scope(&self) -> JsString {
        self.inner.get("scope").as_::<JsString>()
    }

}
impl ServiceWorkerRegistration {
    /// Getter of the `updateViaCache` attribute.
    /// [`ServiceWorkerRegistration.updateViaCache`](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerRegistration/updateViaCache)
    pub fn update_via_cache(&self) -> ServiceWorkerUpdateViaCache {
        self.inner.get("updateViaCache").as_::<ServiceWorkerUpdateViaCache>()
    }

}
impl ServiceWorkerRegistration {
    /// The update method.
    /// [`ServiceWorkerRegistration.update`](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerRegistration/update)
    pub fn update(&self, ) -> Promise<ServiceWorkerRegistration> {
        self.inner.call("update", &[]).as_::<Promise<ServiceWorkerRegistration>>()
    }
}
impl ServiceWorkerRegistration {
    /// The unregister method.
    /// [`ServiceWorkerRegistration.unregister`](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerRegistration/unregister)
    pub fn unregister(&self, ) -> Promise<bool> {
        self.inner.call("unregister", &[]).as_::<Promise<bool>>()
    }
}
impl ServiceWorkerRegistration {
    /// Getter of the `onupdatefound` attribute.
    /// [`ServiceWorkerRegistration.onupdatefound`](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerRegistration/onupdatefound)
    pub fn onupdatefound(&self) -> Any {
        self.inner.get("onupdatefound").as_::<Any>()
    }

    /// Setter of the `onupdatefound` attribute.
    /// [`ServiceWorkerRegistration.onupdatefound`](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerRegistration/onupdatefound)
    pub fn set_onupdatefound(&mut self, value: &Any) {
        self.inner.set("onupdatefound", value);
    }
}
impl ServiceWorkerRegistration {
    /// Getter of the `backgroundFetch` attribute.
    /// [`ServiceWorkerRegistration.backgroundFetch`](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerRegistration/backgroundFetch)
    pub fn background_fetch(&self) -> BackgroundFetchManager {
        self.inner.get("backgroundFetch").as_::<BackgroundFetchManager>()
    }

}
impl ServiceWorkerRegistration {
    /// Getter of the `sync` attribute.
    /// [`ServiceWorkerRegistration.sync`](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerRegistration/sync)
    pub fn sync(&self) -> SyncManager {
        self.inner.get("sync").as_::<SyncManager>()
    }

}
impl ServiceWorkerRegistration {
    /// Getter of the `index` attribute.
    /// [`ServiceWorkerRegistration.index`](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerRegistration/index)
    pub fn index(&self) -> ContentIndex {
        self.inner.get("index").as_::<ContentIndex>()
    }

}
impl ServiceWorkerRegistration {
    /// Getter of the `cookies` attribute.
    /// [`ServiceWorkerRegistration.cookies`](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerRegistration/cookies)
    pub fn cookies(&self) -> CookieStoreManager {
        self.inner.get("cookies").as_::<CookieStoreManager>()
    }

}
impl ServiceWorkerRegistration {
    /// The showNotification method.
    /// [`ServiceWorkerRegistration.showNotification`](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerRegistration/showNotification)
    pub fn show_notification0(&self, title: &JsString) -> Promise<Undefined> {
        self.inner.call("showNotification", &[title.into(), ]).as_::<Promise<Undefined>>()
    }
    /// The showNotification method.
    /// [`ServiceWorkerRegistration.showNotification`](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerRegistration/showNotification)
    pub fn show_notification1(&self, title: &JsString, options: &NotificationOptions) -> Promise<Undefined> {
        self.inner.call("showNotification", &[title.into(), options.into(), ]).as_::<Promise<Undefined>>()
    }
}
impl ServiceWorkerRegistration {
    /// The getNotifications method.
    /// [`ServiceWorkerRegistration.getNotifications`](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerRegistration/getNotifications)
    pub fn get_notifications0(&self, ) -> Promise<TypedArray<Notification>> {
        self.inner.call("getNotifications", &[]).as_::<Promise<TypedArray<Notification>>>()
    }
    /// The getNotifications method.
    /// [`ServiceWorkerRegistration.getNotifications`](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerRegistration/getNotifications)
    pub fn get_notifications1(&self, filter: &GetNotificationOptions) -> Promise<TypedArray<Notification>> {
        self.inner.call("getNotifications", &[filter.into(), ]).as_::<Promise<TypedArray<Notification>>>()
    }
}
impl ServiceWorkerRegistration {
    /// Getter of the `paymentManager` attribute.
    /// [`ServiceWorkerRegistration.paymentManager`](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerRegistration/paymentManager)
    pub fn payment_manager(&self) -> PaymentManager {
        self.inner.get("paymentManager").as_::<PaymentManager>()
    }

}
impl ServiceWorkerRegistration {
    /// Getter of the `periodicSync` attribute.
    /// [`ServiceWorkerRegistration.periodicSync`](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerRegistration/periodicSync)
    pub fn periodic_sync(&self) -> PeriodicSyncManager {
        self.inner.get("periodicSync").as_::<PeriodicSyncManager>()
    }

}
impl ServiceWorkerRegistration {
    /// Getter of the `pushManager` attribute.
    /// [`ServiceWorkerRegistration.pushManager`](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerRegistration/pushManager)
    pub fn push_manager(&self) -> PushManager {
        self.inner.get("pushManager").as_::<PushManager>()
    }

}

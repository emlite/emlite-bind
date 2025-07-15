use super::*;




#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct NotificationOptions {
    inner: emlite::Val,
}
impl FromVal for NotificationOptions {
    fn from_val(v: &emlite::Val) -> Self {
        NotificationOptions { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for NotificationOptions {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for NotificationOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for NotificationOptions {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for NotificationOptions {
    fn as_mut(&mut self) -> &mut emlite::Val {
      &mut self.inner
  }
}
impl From<NotificationOptions> for emlite::Val {
    fn from(s: NotificationOptions) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl NotificationOptions {
    pub fn dir(&self) -> NotificationDirection {
        self.inner.get("dir").as_::<NotificationDirection>()
    }

    pub fn set_dir(&mut self, value: NotificationDirection) {
        self.inner.set("dir", value);
    }

}
impl NotificationOptions {
    pub fn lang(&self) -> DOMString {
        self.inner.get("lang").as_::<DOMString>()
    }

    pub fn set_lang(&mut self, value: DOMString) {
        self.inner.set("lang", value);
    }

}
impl NotificationOptions {
    pub fn body(&self) -> DOMString {
        self.inner.get("body").as_::<DOMString>()
    }

    pub fn set_body(&mut self, value: DOMString) {
        self.inner.set("body", value);
    }

}
impl NotificationOptions {
    pub fn tag(&self) -> DOMString {
        self.inner.get("tag").as_::<DOMString>()
    }

    pub fn set_tag(&mut self, value: DOMString) {
        self.inner.set("tag", value);
    }

}
impl NotificationOptions {
    pub fn image(&self) -> USVString {
        self.inner.get("image").as_::<USVString>()
    }

    pub fn set_image(&mut self, value: USVString) {
        self.inner.set("image", value);
    }

}
impl NotificationOptions {
    pub fn icon(&self) -> USVString {
        self.inner.get("icon").as_::<USVString>()
    }

    pub fn set_icon(&mut self, value: USVString) {
        self.inner.set("icon", value);
    }

}
impl NotificationOptions {
    pub fn badge(&self) -> USVString {
        self.inner.get("badge").as_::<USVString>()
    }

    pub fn set_badge(&mut self, value: USVString) {
        self.inner.set("badge", value);
    }

}
impl NotificationOptions {
    pub fn vibrate(&self) -> Any {
        self.inner.get("vibrate").as_::<Any>()
    }

    pub fn set_vibrate(&mut self, value: Any) {
        self.inner.set("vibrate", value);
    }

}
impl NotificationOptions {
    pub fn timestamp(&self) -> Any {
        self.inner.get("timestamp").as_::<Any>()
    }

    pub fn set_timestamp(&mut self, value: Any) {
        self.inner.set("timestamp", value);
    }

}
impl NotificationOptions {
    pub fn renotify(&self) -> bool {
        self.inner.get("renotify").as_::<bool>()
    }

    pub fn set_renotify(&mut self, value: bool) {
        self.inner.set("renotify", value);
    }

}
impl NotificationOptions {
    pub fn silent(&self) -> bool {
        self.inner.get("silent").as_::<bool>()
    }

    pub fn set_silent(&mut self, value: bool) {
        self.inner.set("silent", value);
    }

}
impl NotificationOptions {
    pub fn require_interaction(&self) -> bool {
        self.inner.get("requireInteraction").as_::<bool>()
    }

    pub fn set_require_interaction(&mut self, value: bool) {
        self.inner.set("requireInteraction", value);
    }

}
impl NotificationOptions {
    pub fn data(&self) -> Any {
        self.inner.get("data").as_::<Any>()
    }

    pub fn set_data(&mut self, value: Any) {
        self.inner.set("data", value);
    }

}
impl NotificationOptions {
    pub fn actions(&self) -> Sequence<NotificationAction> {
        self.inner.get("actions").as_::<Sequence<NotificationAction>>()
    }

    pub fn set_actions(&mut self, value: Sequence<NotificationAction>) {
        self.inner.set("actions", value);
    }

}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct GetNotificationOptions {
    inner: emlite::Val,
}
impl FromVal for GetNotificationOptions {
    fn from_val(v: &emlite::Val) -> Self {
        GetNotificationOptions { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for GetNotificationOptions {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for GetNotificationOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for GetNotificationOptions {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for GetNotificationOptions {
    fn as_mut(&mut self) -> &mut emlite::Val {
      &mut self.inner
  }
}
impl From<GetNotificationOptions> for emlite::Val {
    fn from(s: GetNotificationOptions) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl GetNotificationOptions {
    pub fn tag(&self) -> DOMString {
        self.inner.get("tag").as_::<DOMString>()
    }

    pub fn set_tag(&mut self, value: DOMString) {
        self.inner.set("tag", value);
    }

}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct ServiceWorkerRegistration {
    inner: EventTarget,
}
impl FromVal for ServiceWorkerRegistration {
    fn from_val(v: &emlite::Val) -> Self {
        ServiceWorkerRegistration { inner: EventTarget::from_val(v) }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
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
impl AsRef<emlite::Val> for ServiceWorkerRegistration {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for ServiceWorkerRegistration {
    fn as_mut(&mut self) -> &mut emlite::Val {
      &mut self.inner
  }
}
impl From<ServiceWorkerRegistration> for emlite::Val {
    fn from(s: ServiceWorkerRegistration) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(ServiceWorkerRegistration);


impl ServiceWorkerRegistration {
    pub fn installing(&self) -> ServiceWorker {
        self.inner.get("installing").as_::<ServiceWorker>()
    }

}
impl ServiceWorkerRegistration {
    pub fn waiting(&self) -> ServiceWorker {
        self.inner.get("waiting").as_::<ServiceWorker>()
    }

}
impl ServiceWorkerRegistration {
    pub fn active(&self) -> ServiceWorker {
        self.inner.get("active").as_::<ServiceWorker>()
    }

}
impl ServiceWorkerRegistration {
    pub fn navigation_preload(&self) -> NavigationPreloadManager {
        self.inner.get("navigationPreload").as_::<NavigationPreloadManager>()
    }

}
impl ServiceWorkerRegistration {
    pub fn scope(&self) -> USVString {
        self.inner.get("scope").as_::<USVString>()
    }

}
impl ServiceWorkerRegistration {
    pub fn update_via_cache(&self) -> ServiceWorkerUpdateViaCache {
        self.inner.get("updateViaCache").as_::<ServiceWorkerUpdateViaCache>()
    }

}
impl ServiceWorkerRegistration {
    pub fn update(&self, ) -> Promise {
        self.inner.call("update", &[]).as_::<Promise>()
    }

}
impl ServiceWorkerRegistration {
    pub fn unregister(&self, ) -> Promise {
        self.inner.call("unregister", &[]).as_::<Promise>()
    }

}
impl ServiceWorkerRegistration {
    pub fn onupdatefound(&self) -> Any {
        self.inner.get("onupdatefound").as_::<Any>()
    }

    pub fn set_onupdatefound(&mut self, value: Any) {
        self.inner.set("onupdatefound", value);
    }

}
impl ServiceWorkerRegistration {
    pub fn background_fetch(&self) -> BackgroundFetchManager {
        self.inner.get("backgroundFetch").as_::<BackgroundFetchManager>()
    }

}
impl ServiceWorkerRegistration {
    pub fn sync(&self) -> SyncManager {
        self.inner.get("sync").as_::<SyncManager>()
    }

}
impl ServiceWorkerRegistration {
    pub fn index(&self) -> ContentIndex {
        self.inner.get("index").as_::<ContentIndex>()
    }

}
impl ServiceWorkerRegistration {
    pub fn cookies(&self) -> CookieStoreManager {
        self.inner.get("cookies").as_::<CookieStoreManager>()
    }

}
impl ServiceWorkerRegistration {
    pub fn show_notification0(&self, title: DOMString) -> Promise {
        self.inner.call("showNotification", &[title.into(), ]).as_::<Promise>()
    }

    pub fn show_notification1(&self, title: DOMString, options: NotificationOptions) -> Promise {
        self.inner.call("showNotification", &[title.into(), options.into(), ]).as_::<Promise>()
    }

}
impl ServiceWorkerRegistration {
    pub fn get_notifications0(&self, ) -> Promise {
        self.inner.call("getNotifications", &[]).as_::<Promise>()
    }

    pub fn get_notifications1(&self, filter: GetNotificationOptions) -> Promise {
        self.inner.call("getNotifications", &[filter.into(), ]).as_::<Promise>()
    }

}
impl ServiceWorkerRegistration {
    pub fn payment_manager(&self) -> PaymentManager {
        self.inner.get("paymentManager").as_::<PaymentManager>()
    }

}
impl ServiceWorkerRegistration {
    pub fn periodic_sync(&self) -> PeriodicSyncManager {
        self.inner.get("periodicSync").as_::<PeriodicSyncManager>()
    }

}
impl ServiceWorkerRegistration {
    pub fn push_manager(&self) -> PushManager {
        self.inner.get("pushManager").as_::<PushManager>()
    }

}

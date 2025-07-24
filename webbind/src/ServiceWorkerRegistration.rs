use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct NotificationOptions {
    inner: Any,
}
impl FromVal for NotificationOptions {
    fn from_val(v: &Any) -> Self {
        NotificationOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for NotificationOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for NotificationOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for NotificationOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for NotificationOptions {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<NotificationOptions> for Any {
    fn from(s: NotificationOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&NotificationOptions> for Any {
    fn from(s: &NotificationOptions) -> Any {
        s.inner.clone()
    }
}

impl NotificationOptions {
    pub fn dir(&self) -> NotificationDirection {
        self.inner.get("dir").as_::<NotificationDirection>()
    }

    pub fn set_dir(&mut self, value: &NotificationDirection) {
        self.inner.set("dir", value);
    }
}
impl NotificationOptions {
    pub fn lang(&self) -> DOMString {
        self.inner.get("lang").as_::<DOMString>()
    }

    pub fn set_lang(&mut self, value: &DOMString) {
        self.inner.set("lang", value);
    }
}
impl NotificationOptions {
    pub fn body(&self) -> DOMString {
        self.inner.get("body").as_::<DOMString>()
    }

    pub fn set_body(&mut self, value: &DOMString) {
        self.inner.set("body", value);
    }
}
impl NotificationOptions {
    pub fn tag(&self) -> DOMString {
        self.inner.get("tag").as_::<DOMString>()
    }

    pub fn set_tag(&mut self, value: &DOMString) {
        self.inner.set("tag", value);
    }
}
impl NotificationOptions {
    pub fn image(&self) -> USVString {
        self.inner.get("image").as_::<USVString>()
    }

    pub fn set_image(&mut self, value: &USVString) {
        self.inner.set("image", value);
    }
}
impl NotificationOptions {
    pub fn icon(&self) -> USVString {
        self.inner.get("icon").as_::<USVString>()
    }

    pub fn set_icon(&mut self, value: &USVString) {
        self.inner.set("icon", value);
    }
}
impl NotificationOptions {
    pub fn badge(&self) -> USVString {
        self.inner.get("badge").as_::<USVString>()
    }

    pub fn set_badge(&mut self, value: &USVString) {
        self.inner.set("badge", value);
    }
}
impl NotificationOptions {
    pub fn vibrate(&self) -> Any {
        self.inner.get("vibrate").as_::<Any>()
    }

    pub fn set_vibrate(&mut self, value: &Any) {
        self.inner.set("vibrate", value);
    }
}
impl NotificationOptions {
    pub fn timestamp(&self) -> Any {
        self.inner.get("timestamp").as_::<Any>()
    }

    pub fn set_timestamp(&mut self, value: &Any) {
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

    pub fn set_data(&mut self, value: &Any) {
        self.inner.set("data", value);
    }
}
impl NotificationOptions {
    pub fn actions(&self) -> Sequence<NotificationAction> {
        self.inner
            .get("actions")
            .as_::<Sequence<NotificationAction>>()
    }

    pub fn set_actions(&mut self, value: &Sequence<NotificationAction>) {
        self.inner.set("actions", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct GetNotificationOptions {
    inner: Any,
}
impl FromVal for GetNotificationOptions {
    fn from_val(v: &Any) -> Self {
        GetNotificationOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for GetNotificationOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for GetNotificationOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for GetNotificationOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for GetNotificationOptions {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<GetNotificationOptions> for Any {
    fn from(s: GetNotificationOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&GetNotificationOptions> for Any {
    fn from(s: &GetNotificationOptions) -> Any {
        s.inner.clone()
    }
}

impl GetNotificationOptions {
    pub fn tag(&self) -> DOMString {
        self.inner.get("tag").as_::<DOMString>()
    }

    pub fn set_tag(&mut self, value: &DOMString) {
        self.inner.set("tag", value);
    }
}
/// The ServiceWorkerRegistration class.
/// [`ServiceWorkerRegistration`](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerRegistration)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct ServiceWorkerRegistration {
    inner: EventTarget,
}
impl FromVal for ServiceWorkerRegistration {
    fn from_val(v: &Any) -> Self {
        ServiceWorkerRegistration {
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
        self.inner
            .get("navigationPreload")
            .as_::<NavigationPreloadManager>()
    }
}
impl ServiceWorkerRegistration {
    /// Getter of the `scope` attribute.
    /// [`ServiceWorkerRegistration.scope`](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerRegistration/scope)
    pub fn scope(&self) -> USVString {
        self.inner.get("scope").as_::<USVString>()
    }
}
impl ServiceWorkerRegistration {
    /// Getter of the `updateViaCache` attribute.
    /// [`ServiceWorkerRegistration.updateViaCache`](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerRegistration/updateViaCache)
    pub fn update_via_cache(&self) -> ServiceWorkerUpdateViaCache {
        self.inner
            .get("updateViaCache")
            .as_::<ServiceWorkerUpdateViaCache>()
    }
}
impl ServiceWorkerRegistration {
    /// The update method.
    /// [`ServiceWorkerRegistration.update`](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerRegistration/update)
    pub fn update(&self) -> Promise<ServiceWorkerRegistration> {
        self.inner
            .call("update", &[])
            .as_::<Promise<ServiceWorkerRegistration>>()
    }
}
impl ServiceWorkerRegistration {
    /// The unregister method.
    /// [`ServiceWorkerRegistration.unregister`](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerRegistration/unregister)
    pub fn unregister(&self) -> Promise<bool> {
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
        self.inner
            .get("backgroundFetch")
            .as_::<BackgroundFetchManager>()
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
    pub fn show_notification0(&self, title: &DOMString) -> Promise<Undefined> {
        self.inner
            .call("showNotification", &[title.into()])
            .as_::<Promise<Undefined>>()
    }
    /// The showNotification method.
    /// [`ServiceWorkerRegistration.showNotification`](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerRegistration/showNotification)
    pub fn show_notification1(
        &self,
        title: &DOMString,
        options: &NotificationOptions,
    ) -> Promise<Undefined> {
        self.inner
            .call("showNotification", &[title.into(), options.into()])
            .as_::<Promise<Undefined>>()
    }
}
impl ServiceWorkerRegistration {
    /// The getNotifications method.
    /// [`ServiceWorkerRegistration.getNotifications`](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerRegistration/getNotifications)
    pub fn get_notifications0(&self) -> Promise<Sequence<Notification>> {
        self.inner
            .call("getNotifications", &[])
            .as_::<Promise<Sequence<Notification>>>()
    }
    /// The getNotifications method.
    /// [`ServiceWorkerRegistration.getNotifications`](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerRegistration/getNotifications)
    pub fn get_notifications1(
        &self,
        filter: &GetNotificationOptions,
    ) -> Promise<Sequence<Notification>> {
        self.inner
            .call("getNotifications", &[filter.into()])
            .as_::<Promise<Sequence<Notification>>>()
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

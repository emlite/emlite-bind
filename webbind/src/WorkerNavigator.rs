use super::*;

/// The WorkerNavigator class.
/// [`WorkerNavigator`](https://developer.mozilla.org/en-US/docs/Web/API/WorkerNavigator)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct WorkerNavigator {
    inner: Any,
}

impl FromVal for WorkerNavigator {
    fn from_val(v: &Any) -> Self {
        WorkerNavigator {
            inner: Any::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for WorkerNavigator {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for WorkerNavigator {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for WorkerNavigator {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for WorkerNavigator {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<WorkerNavigator> for Any {
    fn from(s: WorkerNavigator) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&WorkerNavigator> for Any {
    fn from(s: &WorkerNavigator) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(WorkerNavigator);

impl WorkerNavigator {
    /// Getter of the `mediaCapabilities` attribute.
    /// [`WorkerNavigator.mediaCapabilities`](https://developer.mozilla.org/en-US/docs/Web/API/WorkerNavigator/mediaCapabilities)
    pub fn media_capabilities(&self) -> MediaCapabilities {
        self.inner
            .get("mediaCapabilities")
            .as_::<MediaCapabilities>()
    }
}
impl WorkerNavigator {
    /// Getter of the `permissions` attribute.
    /// [`WorkerNavigator.permissions`](https://developer.mozilla.org/en-US/docs/Web/API/WorkerNavigator/permissions)
    pub fn permissions(&self) -> Permissions {
        self.inner.get("permissions").as_::<Permissions>()
    }
}
impl WorkerNavigator {
    /// Getter of the `serial` attribute.
    /// [`WorkerNavigator.serial`](https://developer.mozilla.org/en-US/docs/Web/API/WorkerNavigator/serial)
    pub fn serial(&self) -> Serial {
        self.inner.get("serial").as_::<Serial>()
    }
}
impl WorkerNavigator {
    /// Getter of the `serviceWorker` attribute.
    /// [`WorkerNavigator.serviceWorker`](https://developer.mozilla.org/en-US/docs/Web/API/WorkerNavigator/serviceWorker)
    pub fn service_worker(&self) -> ServiceWorkerContainer {
        self.inner
            .get("serviceWorker")
            .as_::<ServiceWorkerContainer>()
    }
}
impl WorkerNavigator {
    /// Getter of the `hid` attribute.
    /// [`WorkerNavigator.hid`](https://developer.mozilla.org/en-US/docs/Web/API/WorkerNavigator/hid)
    pub fn hid(&self) -> HID {
        self.inner.get("hid").as_::<HID>()
    }
}
impl WorkerNavigator {
    /// Getter of the `usb` attribute.
    /// [`WorkerNavigator.usb`](https://developer.mozilla.org/en-US/docs/Web/API/WorkerNavigator/usb)
    pub fn usb(&self) -> USB {
        self.inner.get("usb").as_::<USB>()
    }
}
impl WorkerNavigator {
    /// Getter of the `deviceMemory` attribute.
    /// [`WorkerNavigator.deviceMemory`](https://developer.mozilla.org/en-US/docs/Web/API/WorkerNavigator/deviceMemory)
    pub fn device_memory(&self) -> f64 {
        self.inner.get("deviceMemory").as_::<f64>()
    }
}
impl WorkerNavigator {
    /// Getter of the `globalPrivacyControl` attribute.
    /// [`WorkerNavigator.globalPrivacyControl`](https://developer.mozilla.org/en-US/docs/Web/API/WorkerNavigator/globalPrivacyControl)
    pub fn global_privacy_control(&self) -> bool {
        self.inner.get("globalPrivacyControl").as_::<bool>()
    }
}
impl WorkerNavigator {
    /// Getter of the `oscpu` attribute.
    /// [`WorkerNavigator.oscpu`](https://developer.mozilla.org/en-US/docs/Web/API/WorkerNavigator/oscpu)
    pub fn oscpu(&self) -> JsString {
        self.inner.get("oscpu").as_::<JsString>()
    }
}
impl WorkerNavigator {
    /// Getter of the `language` attribute.
    /// [`WorkerNavigator.language`](https://developer.mozilla.org/en-US/docs/Web/API/WorkerNavigator/language)
    pub fn language(&self) -> JsString {
        self.inner.get("language").as_::<JsString>()
    }
}
impl WorkerNavigator {
    /// Getter of the `languages` attribute.
    /// [`WorkerNavigator.languages`](https://developer.mozilla.org/en-US/docs/Web/API/WorkerNavigator/languages)
    pub fn languages(&self) -> TypedArray<JsString> {
        self.inner.get("languages").as_::<TypedArray<JsString>>()
    }
}
impl WorkerNavigator {
    /// Getter of the `onLine` attribute.
    /// [`WorkerNavigator.onLine`](https://developer.mozilla.org/en-US/docs/Web/API/WorkerNavigator/onLine)
    pub fn on_line(&self) -> bool {
        self.inner.get("onLine").as_::<bool>()
    }
}
impl WorkerNavigator {
    /// Getter of the `hardwareConcurrency` attribute.
    /// [`WorkerNavigator.hardwareConcurrency`](https://developer.mozilla.org/en-US/docs/Web/API/WorkerNavigator/hardwareConcurrency)
    pub fn hardware_concurrency(&self) -> u64 {
        self.inner.get("hardwareConcurrency").as_::<u64>()
    }
}
impl WorkerNavigator {
    /// Getter of the `connection` attribute.
    /// [`WorkerNavigator.connection`](https://developer.mozilla.org/en-US/docs/Web/API/WorkerNavigator/connection)
    pub fn connection(&self) -> NetworkInformation {
        self.inner.get("connection").as_::<NetworkInformation>()
    }
}
impl WorkerNavigator {
    /// Getter of the `storageBuckets` attribute.
    /// [`WorkerNavigator.storageBuckets`](https://developer.mozilla.org/en-US/docs/Web/API/WorkerNavigator/storageBuckets)
    pub fn storage_buckets(&self) -> StorageBucketManager {
        self.inner
            .get("storageBuckets")
            .as_::<StorageBucketManager>()
    }
}
impl WorkerNavigator {
    /// Getter of the `storage` attribute.
    /// [`WorkerNavigator.storage`](https://developer.mozilla.org/en-US/docs/Web/API/WorkerNavigator/storage)
    pub fn storage(&self) -> StorageManager {
        self.inner.get("storage").as_::<StorageManager>()
    }
}
impl WorkerNavigator {
    /// Getter of the `userAgentData` attribute.
    /// [`WorkerNavigator.userAgentData`](https://developer.mozilla.org/en-US/docs/Web/API/WorkerNavigator/userAgentData)
    pub fn user_agent_data(&self) -> NavigatorUAData {
        self.inner.get("userAgentData").as_::<NavigatorUAData>()
    }
}
impl WorkerNavigator {
    /// Getter of the `locks` attribute.
    /// [`WorkerNavigator.locks`](https://developer.mozilla.org/en-US/docs/Web/API/WorkerNavigator/locks)
    pub fn locks(&self) -> LockManager {
        self.inner.get("locks").as_::<LockManager>()
    }
}
impl WorkerNavigator {
    /// Getter of the `gpu` attribute.
    /// [`WorkerNavigator.gpu`](https://developer.mozilla.org/en-US/docs/Web/API/WorkerNavigator/gpu)
    pub fn gpu(&self) -> GPU {
        self.inner.get("gpu").as_::<GPU>()
    }
}
impl WorkerNavigator {
    /// Getter of the `ml` attribute.
    /// [`WorkerNavigator.ml`](https://developer.mozilla.org/en-US/docs/Web/API/WorkerNavigator/ml)
    pub fn ml(&self) -> ML {
        self.inner.get("ml").as_::<ML>()
    }
}
impl WorkerNavigator {
    /// The setAppBadge method.
    /// [`WorkerNavigator.setAppBadge`](https://developer.mozilla.org/en-US/docs/Web/API/WorkerNavigator/setAppBadge)
    pub fn set_app_badge0(&self) -> Promise<Undefined> {
        self.inner
            .call("setAppBadge", &[])
            .as_::<Promise<Undefined>>()
    }
    /// The setAppBadge method.
    /// [`WorkerNavigator.setAppBadge`](https://developer.mozilla.org/en-US/docs/Web/API/WorkerNavigator/setAppBadge)
    pub fn set_app_badge1(&self, contents: u64) -> Promise<Undefined> {
        self.inner
            .call("setAppBadge", &[contents.into()])
            .as_::<Promise<Undefined>>()
    }
}
impl WorkerNavigator {
    /// The clearAppBadge method.
    /// [`WorkerNavigator.clearAppBadge`](https://developer.mozilla.org/en-US/docs/Web/API/WorkerNavigator/clearAppBadge)
    pub fn clear_app_badge(&self) -> Promise<Undefined> {
        self.inner
            .call("clearAppBadge", &[])
            .as_::<Promise<Undefined>>()
    }
}
impl WorkerNavigator {
    /// The taintEnabled method.
    /// [`WorkerNavigator.taintEnabled`](https://developer.mozilla.org/en-US/docs/Web/API/WorkerNavigator/taintEnabled)
    pub fn taint_enabled(&self) -> bool {
        self.inner.call("taintEnabled", &[]).as_::<bool>()
    }
}

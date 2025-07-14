use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct WorkerNavigator {
    inner: emlite::Val,
}
impl FromVal for WorkerNavigator {
    fn from_val(v: &emlite::Val) -> Self {
        WorkerNavigator {
            inner: emlite::Val::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for WorkerNavigator {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for WorkerNavigator {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for WorkerNavigator {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for WorkerNavigator {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<WorkerNavigator> for emlite::Val {
    fn from(s: WorkerNavigator) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(WorkerNavigator);

impl WorkerNavigator {
    pub fn media_capabilities(&self) -> MediaCapabilities {
        self.inner
            .get("mediaCapabilities")
            .as_::<MediaCapabilities>()
    }
}
impl WorkerNavigator {
    pub fn permissions(&self) -> Permissions {
        self.inner.get("permissions").as_::<Permissions>()
    }
}
impl WorkerNavigator {
    pub fn serial(&self) -> Serial {
        self.inner.get("serial").as_::<Serial>()
    }
}
impl WorkerNavigator {
    pub fn service_worker(&self) -> ServiceWorkerContainer {
        self.inner
            .get("serviceWorker")
            .as_::<ServiceWorkerContainer>()
    }
}
impl WorkerNavigator {
    pub fn hid(&self) -> HID {
        self.inner.get("hid").as_::<HID>()
    }
}
impl WorkerNavigator {
    pub fn usb(&self) -> USB {
        self.inner.get("usb").as_::<USB>()
    }
}
impl WorkerNavigator {
    pub fn set_app_badge0(&self) -> jsbind::Promise {
        self.inner.call("setAppBadge", &[]).as_::<jsbind::Promise>()
    }

    pub fn set_app_badge1(&self, contents: u64) -> jsbind::Promise {
        self.inner
            .call("setAppBadge", &[contents.into()])
            .as_::<jsbind::Promise>()
    }
}
impl WorkerNavigator {
    pub fn clear_app_badge(&self) -> jsbind::Promise {
        self.inner
            .call("clearAppBadge", &[])
            .as_::<jsbind::Promise>()
    }
}
impl WorkerNavigator {
    pub fn device_memory(&self) -> f64 {
        self.inner.get("deviceMemory").as_::<f64>()
    }
}
impl WorkerNavigator {
    pub fn global_privacy_control(&self) -> bool {
        self.inner.get("globalPrivacyControl").as_::<bool>()
    }
}
impl WorkerNavigator {
    pub fn taint_enabled(&self) -> bool {
        self.inner.call("taintEnabled", &[]).as_::<bool>()
    }
}
impl WorkerNavigator {
    pub fn oscpu(&self) -> jsbind::DOMString {
        self.inner.get("oscpu").as_::<jsbind::DOMString>()
    }
}
impl WorkerNavigator {
    pub fn language(&self) -> jsbind::DOMString {
        self.inner.get("language").as_::<jsbind::DOMString>()
    }
}
impl WorkerNavigator {
    pub fn languages(&self) -> jsbind::FrozenArray<jsbind::DOMString> {
        self.inner
            .get("languages")
            .as_::<jsbind::FrozenArray<jsbind::DOMString>>()
    }
}
impl WorkerNavigator {
    pub fn on_line(&self) -> bool {
        self.inner.get("onLine").as_::<bool>()
    }
}
impl WorkerNavigator {
    pub fn hardware_concurrency(&self) -> u64 {
        self.inner.get("hardwareConcurrency").as_::<u64>()
    }
}
impl WorkerNavigator {
    pub fn connection(&self) -> NetworkInformation {
        self.inner.get("connection").as_::<NetworkInformation>()
    }
}
impl WorkerNavigator {
    pub fn storage_buckets(&self) -> StorageBucketManager {
        self.inner
            .get("storageBuckets")
            .as_::<StorageBucketManager>()
    }
}
impl WorkerNavigator {
    pub fn storage(&self) -> StorageManager {
        self.inner.get("storage").as_::<StorageManager>()
    }
}
impl WorkerNavigator {
    pub fn user_agent_data(&self) -> NavigatorUAData {
        self.inner.get("userAgentData").as_::<NavigatorUAData>()
    }
}
impl WorkerNavigator {
    pub fn locks(&self) -> LockManager {
        self.inner.get("locks").as_::<LockManager>()
    }
}
impl WorkerNavigator {
    pub fn gpu(&self) -> GPU {
        self.inner.get("gpu").as_::<GPU>()
    }
}
impl WorkerNavigator {
    pub fn ml(&self) -> ML {
        self.inner.get("ml").as_::<ML>()
    }
}

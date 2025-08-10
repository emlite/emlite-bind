use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct StorageAccessTypes {
    inner: Any,
}
impl FromVal for StorageAccessTypes {
    fn from_val(v: &Any) -> Self {
        StorageAccessTypes { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for StorageAccessTypes {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for StorageAccessTypes {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for StorageAccessTypes {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for StorageAccessTypes {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<StorageAccessTypes> for Any {
    fn from(s: StorageAccessTypes) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&StorageAccessTypes> for Any {
    fn from(s: &StorageAccessTypes) -> Any {
        s.inner.clone()
    }
}

impl StorageAccessTypes {
    pub fn all(&self) -> bool {
        self.inner.get("all").as_::<bool>()
    }

    pub fn set_all(&mut self, value: bool) {
        self.inner.set("all", value);
    }
}
impl StorageAccessTypes {
    pub fn cookies(&self) -> bool {
        self.inner.get("cookies").as_::<bool>()
    }

    pub fn set_cookies(&mut self, value: bool) {
        self.inner.set("cookies", value);
    }
}
impl StorageAccessTypes {
    pub fn session_storage(&self) -> bool {
        self.inner.get("sessionStorage").as_::<bool>()
    }

    pub fn set_session_storage(&mut self, value: bool) {
        self.inner.set("sessionStorage", value);
    }
}
impl StorageAccessTypes {
    pub fn local_storage(&self) -> bool {
        self.inner.get("localStorage").as_::<bool>()
    }

    pub fn set_local_storage(&mut self, value: bool) {
        self.inner.set("localStorage", value);
    }
}
impl StorageAccessTypes {
    pub fn indexed_db(&self) -> bool {
        self.inner.get("indexedDB").as_::<bool>()
    }

    pub fn set_indexed_db(&mut self, value: bool) {
        self.inner.set("indexedDB", value);
    }
}
impl StorageAccessTypes {
    pub fn locks(&self) -> bool {
        self.inner.get("locks").as_::<bool>()
    }

    pub fn set_locks(&mut self, value: bool) {
        self.inner.set("locks", value);
    }
}
impl StorageAccessTypes {
    pub fn caches(&self) -> bool {
        self.inner.get("caches").as_::<bool>()
    }

    pub fn set_caches(&mut self, value: bool) {
        self.inner.set("caches", value);
    }
}
impl StorageAccessTypes {
    pub fn get_directory(&self) -> bool {
        self.inner.get("getDirectory").as_::<bool>()
    }

    pub fn set_get_directory(&mut self, value: bool) {
        self.inner.set("getDirectory", value);
    }
}
impl StorageAccessTypes {
    pub fn estimate(&self) -> bool {
        self.inner.get("estimate").as_::<bool>()
    }

    pub fn set_estimate(&mut self, value: bool) {
        self.inner.set("estimate", value);
    }
}
impl StorageAccessTypes {
    pub fn create_object_url(&self) -> bool {
        self.inner.get("createObjectURL").as_::<bool>()
    }

    pub fn set_create_object_url(&mut self, value: bool) {
        self.inner.set("createObjectURL", value);
    }
}
impl StorageAccessTypes {
    pub fn revoke_object_url(&self) -> bool {
        self.inner.get("revokeObjectURL").as_::<bool>()
    }

    pub fn set_revoke_object_url(&mut self, value: bool) {
        self.inner.set("revokeObjectURL", value);
    }
}
impl StorageAccessTypes {
    pub fn broadcast_channel(&self) -> bool {
        self.inner.get("BroadcastChannel").as_::<bool>()
    }

    pub fn set_broadcast_channel(&mut self, value: bool) {
        self.inner.set("BroadcastChannel", value);
    }
}
impl StorageAccessTypes {
    pub fn shared_worker(&self) -> bool {
        self.inner.get("SharedWorker").as_::<bool>()
    }

    pub fn set_shared_worker(&mut self, value: bool) {
        self.inner.set("SharedWorker", value);
    }
}

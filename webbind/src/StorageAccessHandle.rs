use super::*;

/// The StorageAccessHandle class.
/// [`StorageAccessHandle`](https://developer.mozilla.org/en-US/docs/Web/API/StorageAccessHandle)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct StorageAccessHandle {
    inner: Any,
}
impl FromVal for StorageAccessHandle {
    fn from_val(v: &Any) -> Self {
        StorageAccessHandle {
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
impl core::ops::Deref for StorageAccessHandle {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for StorageAccessHandle {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for StorageAccessHandle {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for StorageAccessHandle {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<StorageAccessHandle> for Any {
    fn from(s: StorageAccessHandle) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&StorageAccessHandle> for Any {
    fn from(s: &StorageAccessHandle) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(StorageAccessHandle);

impl StorageAccessHandle {
    /// Getter of the `sessionStorage` attribute.
    /// [`StorageAccessHandle.sessionStorage`](https://developer.mozilla.org/en-US/docs/Web/API/StorageAccessHandle/sessionStorage)
    pub fn session_storage(&self) -> Storage {
        self.inner.get("sessionStorage").as_::<Storage>()
    }
}
impl StorageAccessHandle {
    /// Getter of the `localStorage` attribute.
    /// [`StorageAccessHandle.localStorage`](https://developer.mozilla.org/en-US/docs/Web/API/StorageAccessHandle/localStorage)
    pub fn local_storage(&self) -> Storage {
        self.inner.get("localStorage").as_::<Storage>()
    }
}
impl StorageAccessHandle {
    /// Getter of the `indexedDB` attribute.
    /// [`StorageAccessHandle.indexedDB`](https://developer.mozilla.org/en-US/docs/Web/API/StorageAccessHandle/indexedDB)
    pub fn indexed_db(&self) -> IDBFactory {
        self.inner.get("indexedDB").as_::<IDBFactory>()
    }
}
impl StorageAccessHandle {
    /// Getter of the `locks` attribute.
    /// [`StorageAccessHandle.locks`](https://developer.mozilla.org/en-US/docs/Web/API/StorageAccessHandle/locks)
    pub fn locks(&self) -> LockManager {
        self.inner.get("locks").as_::<LockManager>()
    }
}
impl StorageAccessHandle {
    /// Getter of the `caches` attribute.
    /// [`StorageAccessHandle.caches`](https://developer.mozilla.org/en-US/docs/Web/API/StorageAccessHandle/caches)
    pub fn caches(&self) -> CacheStorage {
        self.inner.get("caches").as_::<CacheStorage>()
    }
}
impl StorageAccessHandle {
    /// The getDirectory method.
    /// [`StorageAccessHandle.getDirectory`](https://developer.mozilla.org/en-US/docs/Web/API/StorageAccessHandle/getDirectory)
    pub fn get_directory(&self) -> Promise<FileSystemDirectoryHandle> {
        self.inner
            .call("getDirectory", &[])
            .as_::<Promise<FileSystemDirectoryHandle>>()
    }
}
impl StorageAccessHandle {
    /// The estimate method.
    /// [`StorageAccessHandle.estimate`](https://developer.mozilla.org/en-US/docs/Web/API/StorageAccessHandle/estimate)
    pub fn estimate(&self) -> Promise<StorageEstimate> {
        self.inner
            .call("estimate", &[])
            .as_::<Promise<StorageEstimate>>()
    }
}
impl StorageAccessHandle {
    /// The createObjectURL method.
    /// [`StorageAccessHandle.createObjectURL`](https://developer.mozilla.org/en-US/docs/Web/API/StorageAccessHandle/createObjectURL)
    pub fn create_object_url(&self, obj: &Any) -> JsString {
        self.inner
            .call("createObjectURL", &[obj.into()])
            .as_::<JsString>()
    }
}
impl StorageAccessHandle {
    /// The revokeObjectURL method.
    /// [`StorageAccessHandle.revokeObjectURL`](https://developer.mozilla.org/en-US/docs/Web/API/StorageAccessHandle/revokeObjectURL)
    pub fn revoke_object_url(&self, url: &JsString) -> Undefined {
        self.inner
            .call("revokeObjectURL", &[url.into()])
            .as_::<Undefined>()
    }
}
impl StorageAccessHandle {
    /// The BroadcastChannel method.
    /// [`StorageAccessHandle.BroadcastChannel`](https://developer.mozilla.org/en-US/docs/Web/API/StorageAccessHandle/BroadcastChannel)
    pub fn broadcast_channel(&self, name: &JsString) -> BroadcastChannel {
        self.inner
            .call("BroadcastChannel", &[name.into()])
            .as_::<BroadcastChannel>()
    }
}
impl StorageAccessHandle {
    /// The SharedWorker method.
    /// [`StorageAccessHandle.SharedWorker`](https://developer.mozilla.org/en-US/docs/Web/API/StorageAccessHandle/SharedWorker)
    pub fn shared_worker0(&self, script_url: &JsString) -> SharedWorker {
        self.inner
            .call("SharedWorker", &[script_url.into()])
            .as_::<SharedWorker>()
    }
    /// The SharedWorker method.
    /// [`StorageAccessHandle.SharedWorker`](https://developer.mozilla.org/en-US/docs/Web/API/StorageAccessHandle/SharedWorker)
    pub fn shared_worker1(&self, script_url: &JsString, options: &Any) -> SharedWorker {
        self.inner
            .call("SharedWorker", &[script_url.into(), options.into()])
            .as_::<SharedWorker>()
    }
}

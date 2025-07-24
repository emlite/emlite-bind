use super::*;

/// The StorageBucket class.
/// [`StorageBucket`](https://developer.mozilla.org/en-US/docs/Web/API/StorageBucket)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct StorageBucket {
    inner: Any,
}
impl FromVal for StorageBucket {
    fn from_val(v: &Any) -> Self {
        StorageBucket {
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
impl core::ops::Deref for StorageBucket {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for StorageBucket {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for StorageBucket {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for StorageBucket {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<StorageBucket> for Any {
    fn from(s: StorageBucket) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&StorageBucket> for Any {
    fn from(s: &StorageBucket) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(StorageBucket);

impl StorageBucket {
    /// Getter of the `name` attribute.
    /// [`StorageBucket.name`](https://developer.mozilla.org/en-US/docs/Web/API/StorageBucket/name)
    pub fn name(&self) -> DOMString {
        self.inner.get("name").as_::<DOMString>()
    }
}
impl StorageBucket {
    /// The persist method.
    /// [`StorageBucket.persist`](https://developer.mozilla.org/en-US/docs/Web/API/StorageBucket/persist)
    pub fn persist(&self) -> Promise<bool> {
        self.inner.call("persist", &[]).as_::<Promise<bool>>()
    }
}
impl StorageBucket {
    /// The persisted method.
    /// [`StorageBucket.persisted`](https://developer.mozilla.org/en-US/docs/Web/API/StorageBucket/persisted)
    pub fn persisted(&self) -> Promise<bool> {
        self.inner.call("persisted", &[]).as_::<Promise<bool>>()
    }
}
impl StorageBucket {
    /// The estimate method.
    /// [`StorageBucket.estimate`](https://developer.mozilla.org/en-US/docs/Web/API/StorageBucket/estimate)
    pub fn estimate(&self) -> Promise<StorageEstimate> {
        self.inner
            .call("estimate", &[])
            .as_::<Promise<StorageEstimate>>()
    }
}
impl StorageBucket {
    /// The setExpires method.
    /// [`StorageBucket.setExpires`](https://developer.mozilla.org/en-US/docs/Web/API/StorageBucket/setExpires)
    pub fn set_expires(&self, expires: &Any) -> Promise<Undefined> {
        self.inner
            .call("setExpires", &[expires.into()])
            .as_::<Promise<Undefined>>()
    }
}
impl StorageBucket {
    /// The expires method.
    /// [`StorageBucket.expires`](https://developer.mozilla.org/en-US/docs/Web/API/StorageBucket/expires)
    pub fn expires(&self) -> Promise<Any> {
        self.inner.call("expires", &[]).as_::<Promise<Any>>()
    }
}
impl StorageBucket {
    /// Getter of the `indexedDB` attribute.
    /// [`StorageBucket.indexedDB`](https://developer.mozilla.org/en-US/docs/Web/API/StorageBucket/indexedDB)
    pub fn indexed_db(&self) -> IDBFactory {
        self.inner.get("indexedDB").as_::<IDBFactory>()
    }
}
impl StorageBucket {
    /// Getter of the `caches` attribute.
    /// [`StorageBucket.caches`](https://developer.mozilla.org/en-US/docs/Web/API/StorageBucket/caches)
    pub fn caches(&self) -> CacheStorage {
        self.inner.get("caches").as_::<CacheStorage>()
    }
}
impl StorageBucket {
    /// The getDirectory method.
    /// [`StorageBucket.getDirectory`](https://developer.mozilla.org/en-US/docs/Web/API/StorageBucket/getDirectory)
    pub fn get_directory(&self) -> Promise<FileSystemDirectoryHandle> {
        self.inner
            .call("getDirectory", &[])
            .as_::<Promise<FileSystemDirectoryHandle>>()
    }
}

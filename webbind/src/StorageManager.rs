use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct StorageEstimate {
    inner: Any,
}
impl FromVal for StorageEstimate {
    fn from_val(v: &Any) -> Self {
        StorageEstimate { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for StorageEstimate {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for StorageEstimate {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for StorageEstimate {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for StorageEstimate {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<StorageEstimate> for Any {
    fn from(s: StorageEstimate) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&StorageEstimate> for Any {
    fn from(s: &StorageEstimate) -> Any {
        s.inner.clone()
    }
}

impl StorageEstimate {
    pub fn usage(&self) -> u64 {
        self.inner.get("usage").as_::<u64>()
    }

    pub fn set_usage(&mut self, value: u64) {
        self.inner.set("usage", value);
    }
}
impl StorageEstimate {
    pub fn quota(&self) -> u64 {
        self.inner.get("quota").as_::<u64>()
    }

    pub fn set_quota(&mut self, value: u64) {
        self.inner.set("quota", value);
    }
}
/// The StorageManager class.
/// [`StorageManager`](https://developer.mozilla.org/en-US/docs/Web/API/StorageManager)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct StorageManager {
    inner: Any,
}
impl FromVal for StorageManager {
    fn from_val(v: &Any) -> Self {
        StorageManager {
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
impl core::ops::Deref for StorageManager {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for StorageManager {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for StorageManager {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for StorageManager {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<StorageManager> for Any {
    fn from(s: StorageManager) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&StorageManager> for Any {
    fn from(s: &StorageManager) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(StorageManager);

impl StorageManager {
    /// The persisted method.
    /// [`StorageManager.persisted`](https://developer.mozilla.org/en-US/docs/Web/API/StorageManager/persisted)
    pub fn persisted(&self) -> Promise<bool> {
        self.inner.call("persisted", &[]).as_::<Promise<bool>>()
    }
}
impl StorageManager {
    /// The persist method.
    /// [`StorageManager.persist`](https://developer.mozilla.org/en-US/docs/Web/API/StorageManager/persist)
    pub fn persist(&self) -> Promise<bool> {
        self.inner.call("persist", &[]).as_::<Promise<bool>>()
    }
}
impl StorageManager {
    /// The estimate method.
    /// [`StorageManager.estimate`](https://developer.mozilla.org/en-US/docs/Web/API/StorageManager/estimate)
    pub fn estimate(&self) -> Promise<StorageEstimate> {
        self.inner
            .call("estimate", &[])
            .as_::<Promise<StorageEstimate>>()
    }
}
impl StorageManager {
    /// The getDirectory method.
    /// [`StorageManager.getDirectory`](https://developer.mozilla.org/en-US/docs/Web/API/StorageManager/getDirectory)
    pub fn get_directory(&self) -> Promise<FileSystemDirectoryHandle> {
        self.inner
            .call("getDirectory", &[])
            .as_::<Promise<FileSystemDirectoryHandle>>()
    }
}

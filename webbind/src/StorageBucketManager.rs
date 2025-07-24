use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct StorageBucketOptions {
    inner: Any,
}
impl FromVal for StorageBucketOptions {
    fn from_val(v: &Any) -> Self {
        StorageBucketOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for StorageBucketOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for StorageBucketOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for StorageBucketOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for StorageBucketOptions {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<StorageBucketOptions> for Any {
    fn from(s: StorageBucketOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&StorageBucketOptions> for Any {
    fn from(s: &StorageBucketOptions) -> Any {
        s.inner.clone()
    }
}

impl StorageBucketOptions {
    pub fn persisted(&self) -> bool {
        self.inner.get("persisted").as_::<bool>()
    }

    pub fn set_persisted(&mut self, value: bool) {
        self.inner.set("persisted", value);
    }
}
impl StorageBucketOptions {
    pub fn quota(&self) -> u64 {
        self.inner.get("quota").as_::<u64>()
    }

    pub fn set_quota(&mut self, value: u64) {
        self.inner.set("quota", value);
    }
}
impl StorageBucketOptions {
    pub fn expires(&self) -> Any {
        self.inner.get("expires").as_::<Any>()
    }

    pub fn set_expires(&mut self, value: &Any) {
        self.inner.set("expires", value);
    }
}
/// The StorageBucketManager class.
/// [`StorageBucketManager`](https://developer.mozilla.org/en-US/docs/Web/API/StorageBucketManager)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct StorageBucketManager {
    inner: Any,
}
impl FromVal for StorageBucketManager {
    fn from_val(v: &Any) -> Self {
        StorageBucketManager {
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
impl core::ops::Deref for StorageBucketManager {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for StorageBucketManager {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for StorageBucketManager {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for StorageBucketManager {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<StorageBucketManager> for Any {
    fn from(s: StorageBucketManager) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&StorageBucketManager> for Any {
    fn from(s: &StorageBucketManager) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(StorageBucketManager);

impl StorageBucketManager {
    /// The open method.
    /// [`StorageBucketManager.open`](https://developer.mozilla.org/en-US/docs/Web/API/StorageBucketManager/open)
    pub fn open0(&self, name: &DOMString) -> Promise<StorageBucket> {
        self.inner
            .call("open", &[name.into()])
            .as_::<Promise<StorageBucket>>()
    }
    /// The open method.
    /// [`StorageBucketManager.open`](https://developer.mozilla.org/en-US/docs/Web/API/StorageBucketManager/open)
    pub fn open1(
        &self,
        name: &DOMString,
        options: &StorageBucketOptions,
    ) -> Promise<StorageBucket> {
        self.inner
            .call("open", &[name.into(), options.into()])
            .as_::<Promise<StorageBucket>>()
    }
}
impl StorageBucketManager {
    /// The keys method.
    /// [`StorageBucketManager.keys`](https://developer.mozilla.org/en-US/docs/Web/API/StorageBucketManager/keys)
    pub fn keys(&self) -> Promise<Sequence<DOMString>> {
        self.inner
            .call("keys", &[])
            .as_::<Promise<Sequence<DOMString>>>()
    }
}
impl StorageBucketManager {
    /// The delete method.
    /// [`StorageBucketManager.delete`](https://developer.mozilla.org/en-US/docs/Web/API/StorageBucketManager/delete)
    pub fn delete(&self, name: &DOMString) -> Promise<Undefined> {
        self.inner
            .call("delete", &[name.into()])
            .as_::<Promise<Undefined>>()
    }
}

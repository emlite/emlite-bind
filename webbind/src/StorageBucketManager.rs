use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct StorageBucketOptions {
    inner: emlite::Val,
}
impl FromVal for StorageBucketOptions {
    fn from_val(v: &emlite::Val) -> Self {
        StorageBucketOptions { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for StorageBucketOptions {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for StorageBucketOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for StorageBucketOptions {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for StorageBucketOptions {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<StorageBucketOptions> for emlite::Val {
    fn from(s: StorageBucketOptions) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&StorageBucketOptions> for emlite::Val {
    fn from(s: &StorageBucketOptions) -> emlite::Val {
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

    pub fn set_expires(&mut self, value: Any) {
        self.inner.set("expires", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct StorageBucketManager {
    inner: emlite::Val,
}
impl FromVal for StorageBucketManager {
    fn from_val(v: &emlite::Val) -> Self {
        StorageBucketManager {
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
impl core::ops::Deref for StorageBucketManager {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for StorageBucketManager {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for StorageBucketManager {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for StorageBucketManager {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<StorageBucketManager> for emlite::Val {
    fn from(s: StorageBucketManager) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&StorageBucketManager> for emlite::Val {
    fn from(s: &StorageBucketManager) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(StorageBucketManager);

impl StorageBucketManager {
    pub fn open0(&self, name: DOMString) -> Promise {
        self.inner.call("open", &[name.into()]).as_::<Promise>()
    }

    pub fn open1(&self, name: DOMString, options: StorageBucketOptions) -> Promise {
        self.inner
            .call("open", &[name.into(), options.into()])
            .as_::<Promise>()
    }
}
impl StorageBucketManager {
    pub fn keys(&self) -> Promise {
        self.inner.call("keys", &[]).as_::<Promise>()
    }
}
impl StorageBucketManager {
    pub fn delete(&self, name: DOMString) -> Promise {
        self.inner.call("delete", &[name.into()]).as_::<Promise>()
    }
}

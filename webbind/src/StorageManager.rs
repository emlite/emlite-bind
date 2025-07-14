use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct StorageEstimate {
    inner: emlite::Val,
}
impl FromVal for StorageEstimate {
    fn from_val(v: &emlite::Val) -> Self {
        StorageEstimate { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for StorageEstimate {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for StorageEstimate {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<StorageEstimate> for emlite::Val {
    fn from(s: StorageEstimate) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
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
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct StorageManager {
    inner: emlite::Val,
}
impl FromVal for StorageManager {
    fn from_val(v: &emlite::Val) -> Self {
        StorageManager {
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
impl core::ops::Deref for StorageManager {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for StorageManager {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<StorageManager> for emlite::Val {
    fn from(s: StorageManager) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl StorageManager {
    pub fn persisted(&self) -> jsbind::Promise {
        self.inner.call("persisted", &[]).as_::<jsbind::Promise>()
    }
}
impl StorageManager {
    pub fn persist(&self) -> jsbind::Promise {
        self.inner.call("persist", &[]).as_::<jsbind::Promise>()
    }
}
impl StorageManager {
    pub fn estimate(&self) -> jsbind::Promise {
        self.inner.call("estimate", &[]).as_::<jsbind::Promise>()
    }
}
impl StorageManager {
    pub fn get_directory(&self) -> jsbind::Promise {
        self.inner
            .call("getDirectory", &[])
            .as_::<jsbind::Promise>()
    }
}

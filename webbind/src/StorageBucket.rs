use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct StorageBucket {
    inner: emlite::Val,
}
impl FromVal for StorageBucket {
    fn from_val(v: &emlite::Val) -> Self {
        StorageBucket {
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
impl core::ops::Deref for StorageBucket {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for StorageBucket {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for StorageBucket {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for StorageBucket {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<StorageBucket> for emlite::Val {
    fn from(s: StorageBucket) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&StorageBucket> for emlite::Val {
    fn from(s: &StorageBucket) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(StorageBucket);

impl StorageBucket {
    pub fn name(&self) -> DOMString {
        self.inner.get("name").as_::<DOMString>()
    }
}
impl StorageBucket {
    pub fn persist(&self) -> Promise {
        self.inner.call("persist", &[]).as_::<Promise>()
    }
}
impl StorageBucket {
    pub fn persisted(&self) -> Promise {
        self.inner.call("persisted", &[]).as_::<Promise>()
    }
}
impl StorageBucket {
    pub fn estimate(&self) -> Promise {
        self.inner.call("estimate", &[]).as_::<Promise>()
    }
}
impl StorageBucket {
    pub fn set_expires(&self, expires: Any) -> Promise {
        self.inner
            .call("setExpires", &[expires.into()])
            .as_::<Promise>()
    }
}
impl StorageBucket {
    pub fn expires(&self) -> Promise {
        self.inner.call("expires", &[]).as_::<Promise>()
    }
}
impl StorageBucket {
    pub fn indexed_db(&self) -> IDBFactory {
        self.inner.get("indexedDB").as_::<IDBFactory>()
    }
}
impl StorageBucket {
    pub fn caches(&self) -> CacheStorage {
        self.inner.get("caches").as_::<CacheStorage>()
    }
}
impl StorageBucket {
    pub fn get_directory(&self) -> Promise {
        self.inner.call("getDirectory", &[]).as_::<Promise>()
    }
}

use super::*;

#[derive(Clone, Debug)]
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
impl std::ops::Deref for StorageBucket {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for StorageBucket {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<StorageBucket> for emlite::Val {
    fn from(s: StorageBucket) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl StorageBucket {
    pub fn name(&self) -> jsbind::DOMString {
        self.inner.get("name").as_::<jsbind::DOMString>()
    }
}
impl StorageBucket {
    pub fn persist(&self) -> jsbind::Promise {
        self.inner.call("persist", &[]).as_::<jsbind::Promise>()
    }
}
impl StorageBucket {
    pub fn persisted(&self) -> jsbind::Promise {
        self.inner.call("persisted", &[]).as_::<jsbind::Promise>()
    }
}
impl StorageBucket {
    pub fn estimate(&self) -> jsbind::Promise {
        self.inner.call("estimate", &[]).as_::<jsbind::Promise>()
    }
}
impl StorageBucket {
    pub fn set_expires(&self, expires: jsbind::Any) -> jsbind::Promise {
        self.inner
            .call("setExpires", &[expires.into()])
            .as_::<jsbind::Promise>()
    }
}
impl StorageBucket {
    pub fn expires(&self) -> jsbind::Promise {
        self.inner.call("expires", &[]).as_::<jsbind::Promise>()
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
    pub fn get_directory(&self) -> jsbind::Promise {
        self.inner
            .call("getDirectory", &[])
            .as_::<jsbind::Promise>()
    }
}

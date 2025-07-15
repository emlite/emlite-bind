use super::*;




#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct StorageAccessHandle {
    inner: emlite::Val,
}
impl FromVal for StorageAccessHandle {
    fn from_val(v: &emlite::Val) -> Self {
        StorageAccessHandle { inner: emlite::Val::from_val(v) }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for StorageAccessHandle {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for StorageAccessHandle {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for StorageAccessHandle {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for StorageAccessHandle {
    fn as_mut(&mut self) -> &mut emlite::Val {
      &mut self.inner
  }
}
impl From<StorageAccessHandle> for emlite::Val {
    fn from(s: StorageAccessHandle) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(StorageAccessHandle);


impl StorageAccessHandle {
    pub fn session_storage(&self) -> Storage {
        self.inner.get("sessionStorage").as_::<Storage>()
    }

}
impl StorageAccessHandle {
    pub fn local_storage(&self) -> Storage {
        self.inner.get("localStorage").as_::<Storage>()
    }

}
impl StorageAccessHandle {
    pub fn indexed_db(&self) -> IDBFactory {
        self.inner.get("indexedDB").as_::<IDBFactory>()
    }

}
impl StorageAccessHandle {
    pub fn locks(&self) -> LockManager {
        self.inner.get("locks").as_::<LockManager>()
    }

}
impl StorageAccessHandle {
    pub fn caches(&self) -> CacheStorage {
        self.inner.get("caches").as_::<CacheStorage>()
    }

}
impl StorageAccessHandle {
    pub fn get_directory(&self, ) -> Promise {
        self.inner.call("getDirectory", &[]).as_::<Promise>()
    }

}
impl StorageAccessHandle {
    pub fn estimate(&self, ) -> Promise {
        self.inner.call("estimate", &[]).as_::<Promise>()
    }

}
impl StorageAccessHandle {
    pub fn create_object_url(&self, obj: Any) -> DOMString {
        self.inner.call("createObjectURL", &[obj.into(), ]).as_::<DOMString>()
    }

}
impl StorageAccessHandle {
    pub fn revoke_object_url(&self, url: DOMString) -> Undefined {
        self.inner.call("revokeObjectURL", &[url.into(), ]).as_::<Undefined>()
    }

}
impl StorageAccessHandle {
    pub fn broadcast_channel(&self, name: DOMString) -> BroadcastChannel {
        self.inner.call("BroadcastChannel", &[name.into(), ]).as_::<BroadcastChannel>()
    }

}
impl StorageAccessHandle {
    pub fn shared_worker0(&self, script_url: USVString) -> SharedWorker {
        self.inner.call("SharedWorker", &[script_url.into(), ]).as_::<SharedWorker>()
    }

    pub fn shared_worker1(&self, script_url: USVString, options: Any) -> SharedWorker {
        self.inner.call("SharedWorker", &[script_url.into(), options.into(), ]).as_::<SharedWorker>()
    }

}

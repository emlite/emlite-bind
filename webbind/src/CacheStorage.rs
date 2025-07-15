use super::*;




#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct MultiCacheQueryOptions {
    inner: emlite::Val,
}
impl FromVal for MultiCacheQueryOptions {
    fn from_val(v: &emlite::Val) -> Self {
        MultiCacheQueryOptions { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for MultiCacheQueryOptions {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for MultiCacheQueryOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for MultiCacheQueryOptions {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for MultiCacheQueryOptions {
    fn as_mut(&mut self) -> &mut emlite::Val {
      &mut self.inner
  }
}
impl From<MultiCacheQueryOptions> for emlite::Val {
    fn from(s: MultiCacheQueryOptions) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl MultiCacheQueryOptions {
    pub fn cache_name(&self) -> DOMString {
        self.inner.get("cacheName").as_::<DOMString>()
    }

    pub fn set_cache_name(&mut self, value: DOMString) {
        self.inner.set("cacheName", value);
    }

}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct CacheStorage {
    inner: emlite::Val,
}
impl FromVal for CacheStorage {
    fn from_val(v: &emlite::Val) -> Self {
        CacheStorage { inner: emlite::Val::from_val(v) }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for CacheStorage {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for CacheStorage {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for CacheStorage {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for CacheStorage {
    fn as_mut(&mut self) -> &mut emlite::Val {
      &mut self.inner
  }
}
impl From<CacheStorage> for emlite::Val {
    fn from(s: CacheStorage) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(CacheStorage);


impl CacheStorage {
    pub fn match_0(&self, request: Any) -> Promise {
        self.inner.call("match", &[request.into(), ]).as_::<Promise>()
    }

    pub fn match_1(&self, request: Any, options: MultiCacheQueryOptions) -> Promise {
        self.inner.call("match", &[request.into(), options.into(), ]).as_::<Promise>()
    }

}
impl CacheStorage {
    pub fn has(&self, cache_name: DOMString) -> Promise {
        self.inner.call("has", &[cache_name.into(), ]).as_::<Promise>()
    }

}
impl CacheStorage {
    pub fn open(&self, cache_name: DOMString) -> Promise {
        self.inner.call("open", &[cache_name.into(), ]).as_::<Promise>()
    }

}
impl CacheStorage {
    pub fn delete(&self, cache_name: DOMString) -> Promise {
        self.inner.call("delete", &[cache_name.into(), ]).as_::<Promise>()
    }

}
impl CacheStorage {
    pub fn keys(&self, ) -> Promise {
        self.inner.call("keys", &[]).as_::<Promise>()
    }

}

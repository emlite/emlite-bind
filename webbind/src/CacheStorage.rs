use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct MultiCacheQueryOptions {
    inner: Any,
}
impl FromVal for MultiCacheQueryOptions {
    fn from_val(v: &Any) -> Self {
        MultiCacheQueryOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for MultiCacheQueryOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for MultiCacheQueryOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for MultiCacheQueryOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for MultiCacheQueryOptions {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<MultiCacheQueryOptions> for Any {
    fn from(s: MultiCacheQueryOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&MultiCacheQueryOptions> for Any {
    fn from(s: &MultiCacheQueryOptions) -> Any {
        s.inner.clone()
    }
}

impl MultiCacheQueryOptions {
    pub fn cache_name(&self) -> DOMString {
        self.inner.get("cacheName").as_::<DOMString>()
    }

    pub fn set_cache_name(&mut self, value: &DOMString) {
        self.inner.set("cacheName", value);
    }
}
/// The CacheStorage class.
/// [`CacheStorage`](https://developer.mozilla.org/en-US/docs/Web/API/CacheStorage)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct CacheStorage {
    inner: Any,
}
impl FromVal for CacheStorage {
    fn from_val(v: &Any) -> Self {
        CacheStorage {
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
impl core::ops::Deref for CacheStorage {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for CacheStorage {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for CacheStorage {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for CacheStorage {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<CacheStorage> for Any {
    fn from(s: CacheStorage) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&CacheStorage> for Any {
    fn from(s: &CacheStorage) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(CacheStorage);

impl CacheStorage {
    /// The match method.
    /// [`CacheStorage.match`](https://developer.mozilla.org/en-US/docs/Web/API/CacheStorage/match)
    pub fn match_0(&self, request: &Any) -> Promise<Any> {
        self.inner
            .call("match", &[request.into()])
            .as_::<Promise<Any>>()
    }
    /// The match method.
    /// [`CacheStorage.match`](https://developer.mozilla.org/en-US/docs/Web/API/CacheStorage/match)
    pub fn match_1(&self, request: &Any, options: &MultiCacheQueryOptions) -> Promise<Any> {
        self.inner
            .call("match", &[request.into(), options.into()])
            .as_::<Promise<Any>>()
    }
}
impl CacheStorage {
    /// The has method.
    /// [`CacheStorage.has`](https://developer.mozilla.org/en-US/docs/Web/API/CacheStorage/has)
    pub fn has(&self, cache_name: &DOMString) -> Promise<bool> {
        self.inner
            .call("has", &[cache_name.into()])
            .as_::<Promise<bool>>()
    }
}
impl CacheStorage {
    /// The open method.
    /// [`CacheStorage.open`](https://developer.mozilla.org/en-US/docs/Web/API/CacheStorage/open)
    pub fn open(&self, cache_name: &DOMString) -> Promise<Cache> {
        self.inner
            .call("open", &[cache_name.into()])
            .as_::<Promise<Cache>>()
    }
}
impl CacheStorage {
    /// The delete method.
    /// [`CacheStorage.delete`](https://developer.mozilla.org/en-US/docs/Web/API/CacheStorage/delete)
    pub fn delete(&self, cache_name: &DOMString) -> Promise<bool> {
        self.inner
            .call("delete", &[cache_name.into()])
            .as_::<Promise<bool>>()
    }
}
impl CacheStorage {
    /// The keys method.
    /// [`CacheStorage.keys`](https://developer.mozilla.org/en-US/docs/Web/API/CacheStorage/keys)
    pub fn keys(&self) -> Promise<Sequence<DOMString>> {
        self.inner
            .call("keys", &[])
            .as_::<Promise<Sequence<DOMString>>>()
    }
}

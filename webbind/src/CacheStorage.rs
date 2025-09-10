use super::*;

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
    pub fn has(&self, cache_name: &JsString) -> Promise<bool> {
        self.inner
            .call("has", &[cache_name.into()])
            .as_::<Promise<bool>>()
    }
}
impl CacheStorage {
    /// The open method.
    /// [`CacheStorage.open`](https://developer.mozilla.org/en-US/docs/Web/API/CacheStorage/open)
    pub fn open(&self, cache_name: &JsString) -> Promise<Cache> {
        self.inner
            .call("open", &[cache_name.into()])
            .as_::<Promise<Cache>>()
    }
}
impl CacheStorage {
    /// The delete method.
    /// [`CacheStorage.delete`](https://developer.mozilla.org/en-US/docs/Web/API/CacheStorage/delete)
    pub fn delete(&self, cache_name: &JsString) -> Promise<bool> {
        self.inner
            .call("delete", &[cache_name.into()])
            .as_::<Promise<bool>>()
    }
}
impl CacheStorage {
    /// The keys method.
    /// [`CacheStorage.keys`](https://developer.mozilla.org/en-US/docs/Web/API/CacheStorage/keys)
    pub fn keys(&self) -> Promise<TypedArray<JsString>> {
        self.inner
            .call("keys", &[])
            .as_::<Promise<TypedArray<JsString>>>()
    }
}

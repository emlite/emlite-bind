use super::*;




/// The Cache class.
/// [`Cache`](https://developer.mozilla.org/en-US/docs/Web/API/Cache)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct Cache {
    inner: Any,
}

impl FromVal for Cache {
    fn from_val(v: &Any) -> Self {
        Cache { inner: Any::from_val(v) }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for Cache {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for Cache {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for Cache {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for Cache {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<Cache> for Any {
    fn from(s: Cache) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&Cache> for Any {
    fn from(s: &Cache) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(Cache);


impl Cache {
    /// The match method.
    /// [`Cache.match`](https://developer.mozilla.org/en-US/docs/Web/API/Cache/match)
    pub fn match_0(&self, request: &Any) -> Promise<Any> {
        self.inner.call("match", &[request.into(), ]).as_::<Promise<Any>>()
    }
    /// The match method.
    /// [`Cache.match`](https://developer.mozilla.org/en-US/docs/Web/API/Cache/match)
    pub fn match_1(&self, request: &Any, options: &CacheQueryOptions) -> Promise<Any> {
        self.inner.call("match", &[request.into(), options.into(), ]).as_::<Promise<Any>>()
    }
}
impl Cache {
    /// The matchAll method.
    /// [`Cache.matchAll`](https://developer.mozilla.org/en-US/docs/Web/API/Cache/matchAll)
    pub fn match_all0(&self, ) -> Promise<TypedArray<Response>> {
        self.inner.call("matchAll", &[]).as_::<Promise<TypedArray<Response>>>()
    }
    /// The matchAll method.
    /// [`Cache.matchAll`](https://developer.mozilla.org/en-US/docs/Web/API/Cache/matchAll)
    pub fn match_all1(&self, request: &Any) -> Promise<TypedArray<Response>> {
        self.inner.call("matchAll", &[request.into(), ]).as_::<Promise<TypedArray<Response>>>()
    }
    /// The matchAll method.
    /// [`Cache.matchAll`](https://developer.mozilla.org/en-US/docs/Web/API/Cache/matchAll)
    pub fn match_all2(&self, request: &Any, options: &CacheQueryOptions) -> Promise<TypedArray<Response>> {
        self.inner.call("matchAll", &[request.into(), options.into(), ]).as_::<Promise<TypedArray<Response>>>()
    }
}
impl Cache {
    /// The add method.
    /// [`Cache.add`](https://developer.mozilla.org/en-US/docs/Web/API/Cache/add)
    pub fn add(&self, request: &Any) -> Promise<Undefined> {
        self.inner.call("add", &[request.into(), ]).as_::<Promise<Undefined>>()
    }
}
impl Cache {
    /// The addAll method.
    /// [`Cache.addAll`](https://developer.mozilla.org/en-US/docs/Web/API/Cache/addAll)
    pub fn add_all(&self, requests: &TypedArray<Any>) -> Promise<Undefined> {
        self.inner.call("addAll", &[requests.into(), ]).as_::<Promise<Undefined>>()
    }
}
impl Cache {
    /// The put method.
    /// [`Cache.put`](https://developer.mozilla.org/en-US/docs/Web/API/Cache/put)
    pub fn put(&self, request: &Any, response: &Response) -> Promise<Undefined> {
        self.inner.call("put", &[request.into(), response.into(), ]).as_::<Promise<Undefined>>()
    }
}
impl Cache {
    /// The delete method.
    /// [`Cache.delete`](https://developer.mozilla.org/en-US/docs/Web/API/Cache/delete)
    pub fn delete0(&self, request: &Any) -> Promise<bool> {
        self.inner.call("delete", &[request.into(), ]).as_::<Promise<bool>>()
    }
    /// The delete method.
    /// [`Cache.delete`](https://developer.mozilla.org/en-US/docs/Web/API/Cache/delete)
    pub fn delete1(&self, request: &Any, options: &CacheQueryOptions) -> Promise<bool> {
        self.inner.call("delete", &[request.into(), options.into(), ]).as_::<Promise<bool>>()
    }
}
impl Cache {
    /// The keys method.
    /// [`Cache.keys`](https://developer.mozilla.org/en-US/docs/Web/API/Cache/keys)
    pub fn keys0(&self, ) -> Promise<TypedArray<Request>> {
        self.inner.call("keys", &[]).as_::<Promise<TypedArray<Request>>>()
    }
    /// The keys method.
    /// [`Cache.keys`](https://developer.mozilla.org/en-US/docs/Web/API/Cache/keys)
    pub fn keys1(&self, request: &Any) -> Promise<TypedArray<Request>> {
        self.inner.call("keys", &[request.into(), ]).as_::<Promise<TypedArray<Request>>>()
    }
    /// The keys method.
    /// [`Cache.keys`](https://developer.mozilla.org/en-US/docs/Web/API/Cache/keys)
    pub fn keys2(&self, request: &Any, options: &CacheQueryOptions) -> Promise<TypedArray<Request>> {
        self.inner.call("keys", &[request.into(), options.into(), ]).as_::<Promise<TypedArray<Request>>>()
    }
}

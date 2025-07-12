use super::*;

#[derive(Clone, Debug)]
pub struct Cache {
    inner: emlite::Val,
}
impl FromVal for Cache {
    fn from_val(v: &emlite::Val) -> Self {
        Cache {
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
impl std::ops::Deref for Cache {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for Cache {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<Cache> for emlite::Val {
    fn from(s: Cache) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl Cache {
    pub fn match_0(&self, request: jsbind::Any) -> jsbind::Promise {
        self.inner
            .call("match", &[request.into()])
            .as_::<jsbind::Promise>()
    }

    pub fn match_1(&self, request: jsbind::Any, options: CacheQueryOptions) -> jsbind::Promise {
        self.inner
            .call("match", &[request.into(), options.into()])
            .as_::<jsbind::Promise>()
    }
}
impl Cache {
    pub fn match_all0(&self) -> jsbind::Promise {
        self.inner.call("matchAll", &[]).as_::<jsbind::Promise>()
    }

    pub fn match_all1(&self, request: jsbind::Any) -> jsbind::Promise {
        self.inner
            .call("matchAll", &[request.into()])
            .as_::<jsbind::Promise>()
    }

    pub fn match_all2(&self, request: jsbind::Any, options: CacheQueryOptions) -> jsbind::Promise {
        self.inner
            .call("matchAll", &[request.into(), options.into()])
            .as_::<jsbind::Promise>()
    }
}
impl Cache {
    pub fn add(&self, request: jsbind::Any) -> jsbind::Promise {
        self.inner
            .call("add", &[request.into()])
            .as_::<jsbind::Promise>()
    }
}
impl Cache {
    pub fn add_all(&self, requests: jsbind::Sequence<jsbind::Any>) -> jsbind::Promise {
        self.inner
            .call("addAll", &[requests.into()])
            .as_::<jsbind::Promise>()
    }
}
impl Cache {
    pub fn put(&self, request: jsbind::Any, response: Response) -> jsbind::Promise {
        self.inner
            .call("put", &[request.into(), response.into()])
            .as_::<jsbind::Promise>()
    }
}
impl Cache {
    pub fn delete0(&self, request: jsbind::Any) -> jsbind::Promise {
        self.inner
            .call("delete", &[request.into()])
            .as_::<jsbind::Promise>()
    }

    pub fn delete1(&self, request: jsbind::Any, options: CacheQueryOptions) -> jsbind::Promise {
        self.inner
            .call("delete", &[request.into(), options.into()])
            .as_::<jsbind::Promise>()
    }
}
impl Cache {
    pub fn keys0(&self) -> jsbind::Promise {
        self.inner.call("keys", &[]).as_::<jsbind::Promise>()
    }

    pub fn keys1(&self, request: jsbind::Any) -> jsbind::Promise {
        self.inner
            .call("keys", &[request.into()])
            .as_::<jsbind::Promise>()
    }

    pub fn keys2(&self, request: jsbind::Any, options: CacheQueryOptions) -> jsbind::Promise {
        self.inner
            .call("keys", &[request.into(), options.into()])
            .as_::<jsbind::Promise>()
    }
}

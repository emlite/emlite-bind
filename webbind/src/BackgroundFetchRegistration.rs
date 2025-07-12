use super::*;

#[derive(Clone, Debug)]
pub struct CacheQueryOptions {
    inner: emlite::Val,
}
impl FromVal for CacheQueryOptions {
    fn from_val(v: &emlite::Val) -> Self {
        CacheQueryOptions { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for CacheQueryOptions {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for CacheQueryOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<CacheQueryOptions> for emlite::Val {
    fn from(s: CacheQueryOptions) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl CacheQueryOptions {
    pub fn ignore_search(&self) -> bool {
        self.inner.get("ignoreSearch").as_::<bool>()
    }

    pub fn set_ignore_search(&mut self, value: bool) {
        self.inner.set("ignoreSearch", value);
    }
}
impl CacheQueryOptions {
    pub fn ignore_method(&self) -> bool {
        self.inner.get("ignoreMethod").as_::<bool>()
    }

    pub fn set_ignore_method(&mut self, value: bool) {
        self.inner.set("ignoreMethod", value);
    }
}
impl CacheQueryOptions {
    pub fn ignore_vary(&self) -> bool {
        self.inner.get("ignoreVary").as_::<bool>()
    }

    pub fn set_ignore_vary(&mut self, value: bool) {
        self.inner.set("ignoreVary", value);
    }
}
#[derive(Clone, Debug)]
pub struct BackgroundFetchRegistration {
    inner: EventTarget,
}
impl FromVal for BackgroundFetchRegistration {
    fn from_val(v: &emlite::Val) -> Self {
        BackgroundFetchRegistration {
            inner: EventTarget::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for BackgroundFetchRegistration {
    type Target = EventTarget;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for BackgroundFetchRegistration {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<BackgroundFetchRegistration> for emlite::Val {
    fn from(s: BackgroundFetchRegistration) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl BackgroundFetchRegistration {
    pub fn id(&self) -> jsbind::DOMString {
        self.inner.get("id").as_::<jsbind::DOMString>()
    }
}
impl BackgroundFetchRegistration {
    pub fn upload_total(&self) -> u64 {
        self.inner.get("uploadTotal").as_::<u64>()
    }
}
impl BackgroundFetchRegistration {
    pub fn uploaded(&self) -> u64 {
        self.inner.get("uploaded").as_::<u64>()
    }
}
impl BackgroundFetchRegistration {
    pub fn download_total(&self) -> u64 {
        self.inner.get("downloadTotal").as_::<u64>()
    }
}
impl BackgroundFetchRegistration {
    pub fn downloaded(&self) -> u64 {
        self.inner.get("downloaded").as_::<u64>()
    }
}
impl BackgroundFetchRegistration {
    pub fn result(&self) -> BackgroundFetchResult {
        self.inner.get("result").as_::<BackgroundFetchResult>()
    }
}
impl BackgroundFetchRegistration {
    pub fn failure_reason(&self) -> BackgroundFetchFailureReason {
        self.inner
            .get("failureReason")
            .as_::<BackgroundFetchFailureReason>()
    }
}
impl BackgroundFetchRegistration {
    pub fn records_available(&self) -> bool {
        self.inner.get("recordsAvailable").as_::<bool>()
    }
}
impl BackgroundFetchRegistration {
    pub fn onprogress(&self) -> jsbind::Any {
        self.inner.get("onprogress").as_::<jsbind::Any>()
    }

    pub fn set_onprogress(&mut self, value: jsbind::Any) {
        self.inner.set("onprogress", value);
    }
}
impl BackgroundFetchRegistration {
    pub fn abort(&self) -> jsbind::Promise {
        self.inner.call("abort", &[]).as_::<jsbind::Promise>()
    }
}
impl BackgroundFetchRegistration {
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
impl BackgroundFetchRegistration {
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

use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
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
impl core::ops::Deref for CacheQueryOptions {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for CacheQueryOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for CacheQueryOptions {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for CacheQueryOptions {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<CacheQueryOptions> for emlite::Val {
    fn from(s: CacheQueryOptions) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&CacheQueryOptions> for emlite::Val {
    fn from(s: &CacheQueryOptions) -> emlite::Val {
        s.inner.clone()
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
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
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
impl core::ops::Deref for BackgroundFetchRegistration {
    type Target = EventTarget;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for BackgroundFetchRegistration {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for BackgroundFetchRegistration {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for BackgroundFetchRegistration {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<BackgroundFetchRegistration> for emlite::Val {
    fn from(s: BackgroundFetchRegistration) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&BackgroundFetchRegistration> for emlite::Val {
    fn from(s: &BackgroundFetchRegistration) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(BackgroundFetchRegistration);

impl BackgroundFetchRegistration {
    pub fn id(&self) -> DOMString {
        self.inner.get("id").as_::<DOMString>()
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
    pub fn onprogress(&self) -> Any {
        self.inner.get("onprogress").as_::<Any>()
    }

    pub fn set_onprogress(&mut self, value: Any) {
        self.inner.set("onprogress", value);
    }
}
impl BackgroundFetchRegistration {
    pub fn abort(&self) -> Promise {
        self.inner.call("abort", &[]).as_::<Promise>()
    }
}
impl BackgroundFetchRegistration {
    pub fn match_0(&self, request: Any) -> Promise {
        self.inner.call("match", &[request.into()]).as_::<Promise>()
    }

    pub fn match_1(&self, request: Any, options: CacheQueryOptions) -> Promise {
        self.inner
            .call("match", &[request.into(), options.into()])
            .as_::<Promise>()
    }
}
impl BackgroundFetchRegistration {
    pub fn match_all0(&self) -> Promise {
        self.inner.call("matchAll", &[]).as_::<Promise>()
    }

    pub fn match_all1(&self, request: Any) -> Promise {
        self.inner
            .call("matchAll", &[request.into()])
            .as_::<Promise>()
    }

    pub fn match_all2(&self, request: Any, options: CacheQueryOptions) -> Promise {
        self.inner
            .call("matchAll", &[request.into(), options.into()])
            .as_::<Promise>()
    }
}

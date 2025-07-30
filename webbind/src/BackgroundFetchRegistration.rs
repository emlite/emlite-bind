use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct CacheQueryOptions {
    inner: Any,
}
impl FromVal for CacheQueryOptions {
    fn from_val(v: &Any) -> Self {
        CacheQueryOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for CacheQueryOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for CacheQueryOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for CacheQueryOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for CacheQueryOptions {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<CacheQueryOptions> for Any {
    fn from(s: CacheQueryOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&CacheQueryOptions> for Any {
    fn from(s: &CacheQueryOptions) -> Any {
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
/// The BackgroundFetchRegistration class.
/// [`BackgroundFetchRegistration`](https://developer.mozilla.org/en-US/docs/Web/API/BackgroundFetchRegistration)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct BackgroundFetchRegistration {
    inner: EventTarget,
}
impl FromVal for BackgroundFetchRegistration {
    fn from_val(v: &Any) -> Self {
        BackgroundFetchRegistration {
            inner: EventTarget::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
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
impl AsRef<Any> for BackgroundFetchRegistration {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for BackgroundFetchRegistration {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<BackgroundFetchRegistration> for Any {
    fn from(s: BackgroundFetchRegistration) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&BackgroundFetchRegistration> for Any {
    fn from(s: &BackgroundFetchRegistration) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(BackgroundFetchRegistration);

impl BackgroundFetchRegistration {
    /// Getter of the `id` attribute.
    /// [`BackgroundFetchRegistration.id`](https://developer.mozilla.org/en-US/docs/Web/API/BackgroundFetchRegistration/id)
    pub fn id(&self) -> JsString {
        self.inner.get("id").as_::<JsString>()
    }
}
impl BackgroundFetchRegistration {
    /// Getter of the `uploadTotal` attribute.
    /// [`BackgroundFetchRegistration.uploadTotal`](https://developer.mozilla.org/en-US/docs/Web/API/BackgroundFetchRegistration/uploadTotal)
    pub fn upload_total(&self) -> u64 {
        self.inner.get("uploadTotal").as_::<u64>()
    }
}
impl BackgroundFetchRegistration {
    /// Getter of the `uploaded` attribute.
    /// [`BackgroundFetchRegistration.uploaded`](https://developer.mozilla.org/en-US/docs/Web/API/BackgroundFetchRegistration/uploaded)
    pub fn uploaded(&self) -> u64 {
        self.inner.get("uploaded").as_::<u64>()
    }
}
impl BackgroundFetchRegistration {
    /// Getter of the `downloadTotal` attribute.
    /// [`BackgroundFetchRegistration.downloadTotal`](https://developer.mozilla.org/en-US/docs/Web/API/BackgroundFetchRegistration/downloadTotal)
    pub fn download_total(&self) -> u64 {
        self.inner.get("downloadTotal").as_::<u64>()
    }
}
impl BackgroundFetchRegistration {
    /// Getter of the `downloaded` attribute.
    /// [`BackgroundFetchRegistration.downloaded`](https://developer.mozilla.org/en-US/docs/Web/API/BackgroundFetchRegistration/downloaded)
    pub fn downloaded(&self) -> u64 {
        self.inner.get("downloaded").as_::<u64>()
    }
}
impl BackgroundFetchRegistration {
    /// Getter of the `result` attribute.
    /// [`BackgroundFetchRegistration.result`](https://developer.mozilla.org/en-US/docs/Web/API/BackgroundFetchRegistration/result)
    pub fn result(&self) -> BackgroundFetchResult {
        self.inner.get("result").as_::<BackgroundFetchResult>()
    }
}
impl BackgroundFetchRegistration {
    /// Getter of the `failureReason` attribute.
    /// [`BackgroundFetchRegistration.failureReason`](https://developer.mozilla.org/en-US/docs/Web/API/BackgroundFetchRegistration/failureReason)
    pub fn failure_reason(&self) -> BackgroundFetchFailureReason {
        self.inner
            .get("failureReason")
            .as_::<BackgroundFetchFailureReason>()
    }
}
impl BackgroundFetchRegistration {
    /// Getter of the `recordsAvailable` attribute.
    /// [`BackgroundFetchRegistration.recordsAvailable`](https://developer.mozilla.org/en-US/docs/Web/API/BackgroundFetchRegistration/recordsAvailable)
    pub fn records_available(&self) -> bool {
        self.inner.get("recordsAvailable").as_::<bool>()
    }
}
impl BackgroundFetchRegistration {
    /// Getter of the `onprogress` attribute.
    /// [`BackgroundFetchRegistration.onprogress`](https://developer.mozilla.org/en-US/docs/Web/API/BackgroundFetchRegistration/onprogress)
    pub fn onprogress(&self) -> Any {
        self.inner.get("onprogress").as_::<Any>()
    }

    /// Setter of the `onprogress` attribute.
    /// [`BackgroundFetchRegistration.onprogress`](https://developer.mozilla.org/en-US/docs/Web/API/BackgroundFetchRegistration/onprogress)
    pub fn set_onprogress(&mut self, value: &Any) {
        self.inner.set("onprogress", value);
    }
}
impl BackgroundFetchRegistration {
    /// The abort method.
    /// [`BackgroundFetchRegistration.abort`](https://developer.mozilla.org/en-US/docs/Web/API/BackgroundFetchRegistration/abort)
    pub fn abort(&self) -> Promise<bool> {
        self.inner.call("abort", &[]).as_::<Promise<bool>>()
    }
}
impl BackgroundFetchRegistration {
    /// The match method.
    /// [`BackgroundFetchRegistration.match`](https://developer.mozilla.org/en-US/docs/Web/API/BackgroundFetchRegistration/match)
    pub fn match_0(&self, request: &Any) -> Promise<BackgroundFetchRecord> {
        self.inner
            .call("match", &[request.into()])
            .as_::<Promise<BackgroundFetchRecord>>()
    }
    /// The match method.
    /// [`BackgroundFetchRegistration.match`](https://developer.mozilla.org/en-US/docs/Web/API/BackgroundFetchRegistration/match)
    pub fn match_1(
        &self,
        request: &Any,
        options: &CacheQueryOptions,
    ) -> Promise<BackgroundFetchRecord> {
        self.inner
            .call("match", &[request.into(), options.into()])
            .as_::<Promise<BackgroundFetchRecord>>()
    }
}
impl BackgroundFetchRegistration {
    /// The matchAll method.
    /// [`BackgroundFetchRegistration.matchAll`](https://developer.mozilla.org/en-US/docs/Web/API/BackgroundFetchRegistration/matchAll)
    pub fn match_all0(&self) -> Promise<TypedArray<BackgroundFetchRecord>> {
        self.inner
            .call("matchAll", &[])
            .as_::<Promise<TypedArray<BackgroundFetchRecord>>>()
    }
    /// The matchAll method.
    /// [`BackgroundFetchRegistration.matchAll`](https://developer.mozilla.org/en-US/docs/Web/API/BackgroundFetchRegistration/matchAll)
    pub fn match_all1(&self, request: &Any) -> Promise<TypedArray<BackgroundFetchRecord>> {
        self.inner
            .call("matchAll", &[request.into()])
            .as_::<Promise<TypedArray<BackgroundFetchRecord>>>()
    }
    /// The matchAll method.
    /// [`BackgroundFetchRegistration.matchAll`](https://developer.mozilla.org/en-US/docs/Web/API/BackgroundFetchRegistration/matchAll)
    pub fn match_all2(
        &self,
        request: &Any,
        options: &CacheQueryOptions,
    ) -> Promise<TypedArray<BackgroundFetchRecord>> {
        self.inner
            .call("matchAll", &[request.into(), options.into()])
            .as_::<Promise<TypedArray<BackgroundFetchRecord>>>()
    }
}

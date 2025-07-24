use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct BackgroundFetchOptions {
    inner: Any,
}
impl FromVal for BackgroundFetchOptions {
    fn from_val(v: &Any) -> Self {
        BackgroundFetchOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for BackgroundFetchOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for BackgroundFetchOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for BackgroundFetchOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for BackgroundFetchOptions {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<BackgroundFetchOptions> for Any {
    fn from(s: BackgroundFetchOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&BackgroundFetchOptions> for Any {
    fn from(s: &BackgroundFetchOptions) -> Any {
        s.inner.clone()
    }
}

impl BackgroundFetchOptions {
    pub fn download_total(&self) -> u64 {
        self.inner.get("downloadTotal").as_::<u64>()
    }

    pub fn set_download_total(&mut self, value: u64) {
        self.inner.set("downloadTotal", value);
    }
}
/// The BackgroundFetchManager class.
/// [`BackgroundFetchManager`](https://developer.mozilla.org/en-US/docs/Web/API/BackgroundFetchManager)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct BackgroundFetchManager {
    inner: Any,
}
impl FromVal for BackgroundFetchManager {
    fn from_val(v: &Any) -> Self {
        BackgroundFetchManager {
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
impl core::ops::Deref for BackgroundFetchManager {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for BackgroundFetchManager {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for BackgroundFetchManager {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for BackgroundFetchManager {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<BackgroundFetchManager> for Any {
    fn from(s: BackgroundFetchManager) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&BackgroundFetchManager> for Any {
    fn from(s: &BackgroundFetchManager) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(BackgroundFetchManager);

impl BackgroundFetchManager {
    /// The fetch method.
    /// [`BackgroundFetchManager.fetch`](https://developer.mozilla.org/en-US/docs/Web/API/BackgroundFetchManager/fetch)
    pub fn fetch0(&self, id: &DOMString, requests: &Any) -> Promise<BackgroundFetchRegistration> {
        self.inner
            .call("fetch", &[id.into(), requests.into()])
            .as_::<Promise<BackgroundFetchRegistration>>()
    }
    /// The fetch method.
    /// [`BackgroundFetchManager.fetch`](https://developer.mozilla.org/en-US/docs/Web/API/BackgroundFetchManager/fetch)
    pub fn fetch1(
        &self,
        id: &DOMString,
        requests: &Any,
        options: &BackgroundFetchOptions,
    ) -> Promise<BackgroundFetchRegistration> {
        self.inner
            .call("fetch", &[id.into(), requests.into(), options.into()])
            .as_::<Promise<BackgroundFetchRegistration>>()
    }
}
impl BackgroundFetchManager {
    /// The get method.
    /// [`BackgroundFetchManager.get`](https://developer.mozilla.org/en-US/docs/Web/API/BackgroundFetchManager/get)
    pub fn get(&self, id: &DOMString) -> Promise<BackgroundFetchRegistration> {
        self.inner
            .call("get", &[id.into()])
            .as_::<Promise<BackgroundFetchRegistration>>()
    }
}
impl BackgroundFetchManager {
    /// The getIds method.
    /// [`BackgroundFetchManager.getIds`](https://developer.mozilla.org/en-US/docs/Web/API/BackgroundFetchManager/getIds)
    pub fn get_ids(&self) -> Promise<Sequence<DOMString>> {
        self.inner
            .call("getIds", &[])
            .as_::<Promise<Sequence<DOMString>>>()
    }
}

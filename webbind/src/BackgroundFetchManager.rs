use super::*;




/// The BackgroundFetchManager class.
/// [`BackgroundFetchManager`](https://developer.mozilla.org/en-US/docs/Web/API/BackgroundFetchManager)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct BackgroundFetchManager {
    inner: Any,
}

impl FromVal for BackgroundFetchManager {
    fn from_val(v: &Any) -> Self {
        BackgroundFetchManager { inner: Any::from_val(v) }
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
    pub fn fetch0(&self, id: &JsString, requests: &Any) -> Promise<BackgroundFetchRegistration> {
        self.inner.call("fetch", &[id.into(), requests.into(), ]).as_::<Promise<BackgroundFetchRegistration>>()
    }
    /// The fetch method.
    /// [`BackgroundFetchManager.fetch`](https://developer.mozilla.org/en-US/docs/Web/API/BackgroundFetchManager/fetch)
    pub fn fetch1(&self, id: &JsString, requests: &Any, options: &BackgroundFetchOptions) -> Promise<BackgroundFetchRegistration> {
        self.inner.call("fetch", &[id.into(), requests.into(), options.into(), ]).as_::<Promise<BackgroundFetchRegistration>>()
    }
}
impl BackgroundFetchManager {
    /// The get method.
    /// [`BackgroundFetchManager.get`](https://developer.mozilla.org/en-US/docs/Web/API/BackgroundFetchManager/get)
    pub fn get(&self, id: &JsString) -> Promise<BackgroundFetchRegistration> {
        self.inner.call("get", &[id.into(), ]).as_::<Promise<BackgroundFetchRegistration>>()
    }
}
impl BackgroundFetchManager {
    /// The getIds method.
    /// [`BackgroundFetchManager.getIds`](https://developer.mozilla.org/en-US/docs/Web/API/BackgroundFetchManager/getIds)
    pub fn get_ids(&self, ) -> Promise<TypedArray<JsString>> {
        self.inner.call("getIds", &[]).as_::<Promise<TypedArray<JsString>>>()
    }
}

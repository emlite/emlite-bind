use super::*;

/// The NavigationPreloadManager class.
/// [`NavigationPreloadManager`](https://developer.mozilla.org/en-US/docs/Web/API/NavigationPreloadManager)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct NavigationPreloadManager {
    inner: Any,
}

impl FromVal for NavigationPreloadManager {
    fn from_val(v: &Any) -> Self {
        NavigationPreloadManager {
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

impl core::ops::Deref for NavigationPreloadManager {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for NavigationPreloadManager {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for NavigationPreloadManager {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for NavigationPreloadManager {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<NavigationPreloadManager> for Any {
    fn from(s: NavigationPreloadManager) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&NavigationPreloadManager> for Any {
    fn from(s: &NavigationPreloadManager) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(NavigationPreloadManager);

impl NavigationPreloadManager {
    /// The enable method.
    /// [`NavigationPreloadManager.enable`](https://developer.mozilla.org/en-US/docs/Web/API/NavigationPreloadManager/enable)
    pub fn enable(&self) -> Promise<Undefined> {
        self.inner.call("enable", &[]).as_::<Promise<Undefined>>()
    }
}
impl NavigationPreloadManager {
    /// The disable method.
    /// [`NavigationPreloadManager.disable`](https://developer.mozilla.org/en-US/docs/Web/API/NavigationPreloadManager/disable)
    pub fn disable(&self) -> Promise<Undefined> {
        self.inner.call("disable", &[]).as_::<Promise<Undefined>>()
    }
}
impl NavigationPreloadManager {
    /// The setHeaderValue method.
    /// [`NavigationPreloadManager.setHeaderValue`](https://developer.mozilla.org/en-US/docs/Web/API/NavigationPreloadManager/setHeaderValue)
    pub fn set_header_value(&self, value: &JsString) -> Promise<Undefined> {
        self.inner
            .call("setHeaderValue", &[value.into()])
            .as_::<Promise<Undefined>>()
    }
}
impl NavigationPreloadManager {
    /// The getState method.
    /// [`NavigationPreloadManager.getState`](https://developer.mozilla.org/en-US/docs/Web/API/NavigationPreloadManager/getState)
    pub fn get_state(&self) -> Promise<NavigationPreloadState> {
        self.inner
            .call("getState", &[])
            .as_::<Promise<NavigationPreloadState>>()
    }
}

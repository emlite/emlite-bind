use super::*;




/// The PeriodicSyncManager class.
/// [`PeriodicSyncManager`](https://developer.mozilla.org/en-US/docs/Web/API/PeriodicSyncManager)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct PeriodicSyncManager {
    inner: Any,
}

impl FromVal for PeriodicSyncManager {
    fn from_val(v: &Any) -> Self {
        PeriodicSyncManager { inner: Any::from_val(v) }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for PeriodicSyncManager {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for PeriodicSyncManager {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for PeriodicSyncManager {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for PeriodicSyncManager {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<PeriodicSyncManager> for Any {
    fn from(s: PeriodicSyncManager) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&PeriodicSyncManager> for Any {
    fn from(s: &PeriodicSyncManager) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(PeriodicSyncManager);


impl PeriodicSyncManager {
    /// The register method.
    /// [`PeriodicSyncManager.register`](https://developer.mozilla.org/en-US/docs/Web/API/PeriodicSyncManager/register)
    pub fn register0(&self, tag: &JsString) -> Promise<Undefined> {
        self.inner.call("register", &[tag.into(), ]).as_::<Promise<Undefined>>()
    }
    /// The register method.
    /// [`PeriodicSyncManager.register`](https://developer.mozilla.org/en-US/docs/Web/API/PeriodicSyncManager/register)
    pub fn register1(&self, tag: &JsString, options: &BackgroundSyncOptions) -> Promise<Undefined> {
        self.inner.call("register", &[tag.into(), options.into(), ]).as_::<Promise<Undefined>>()
    }
}
impl PeriodicSyncManager {
    /// The getTags method.
    /// [`PeriodicSyncManager.getTags`](https://developer.mozilla.org/en-US/docs/Web/API/PeriodicSyncManager/getTags)
    pub fn get_tags(&self, ) -> Promise<TypedArray<JsString>> {
        self.inner.call("getTags", &[]).as_::<Promise<TypedArray<JsString>>>()
    }
}
impl PeriodicSyncManager {
    /// The unregister method.
    /// [`PeriodicSyncManager.unregister`](https://developer.mozilla.org/en-US/docs/Web/API/PeriodicSyncManager/unregister)
    pub fn unregister(&self, tag: &JsString) -> Promise<Undefined> {
        self.inner.call("unregister", &[tag.into(), ]).as_::<Promise<Undefined>>()
    }
}

use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct BackgroundSyncOptions {
    inner: Any,
}
impl FromVal for BackgroundSyncOptions {
    fn from_val(v: &Any) -> Self {
        BackgroundSyncOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for BackgroundSyncOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for BackgroundSyncOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for BackgroundSyncOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for BackgroundSyncOptions {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<BackgroundSyncOptions> for Any {
    fn from(s: BackgroundSyncOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&BackgroundSyncOptions> for Any {
    fn from(s: &BackgroundSyncOptions) -> Any {
        s.inner.clone()
    }
}

impl BackgroundSyncOptions {
    pub fn min_interval(&self) -> u64 {
        self.inner.get("minInterval").as_::<u64>()
    }

    pub fn set_min_interval(&mut self, value: u64) {
        self.inner.set("minInterval", value);
    }
}
/// The PeriodicSyncManager class.
/// [`PeriodicSyncManager`](https://developer.mozilla.org/en-US/docs/Web/API/PeriodicSyncManager)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct PeriodicSyncManager {
    inner: Any,
}
impl FromVal for PeriodicSyncManager {
    fn from_val(v: &Any) -> Self {
        PeriodicSyncManager {
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
    pub fn register0(&self, tag: &str) -> Promise {
        self.inner.call("register", &[tag.into()]).as_::<Promise>()
    }
    /// The register method.
    /// [`PeriodicSyncManager.register`](https://developer.mozilla.org/en-US/docs/Web/API/PeriodicSyncManager/register)
    pub fn register1(&self, tag: &str, options: &BackgroundSyncOptions) -> Promise {
        self.inner
            .call("register", &[tag.into(), options.into()])
            .as_::<Promise>()
    }
}
impl PeriodicSyncManager {
    /// The getTags method.
    /// [`PeriodicSyncManager.getTags`](https://developer.mozilla.org/en-US/docs/Web/API/PeriodicSyncManager/getTags)
    pub fn get_tags(&self) -> Promise {
        self.inner.call("getTags", &[]).as_::<Promise>()
    }
}
impl PeriodicSyncManager {
    /// The unregister method.
    /// [`PeriodicSyncManager.unregister`](https://developer.mozilla.org/en-US/docs/Web/API/PeriodicSyncManager/unregister)
    pub fn unregister(&self, tag: &str) -> Promise {
        self.inner
            .call("unregister", &[tag.into()])
            .as_::<Promise>()
    }
}

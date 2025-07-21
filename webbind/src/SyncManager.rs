use super::*;

/// The SyncManager class.
/// [`SyncManager`](https://developer.mozilla.org/en-US/docs/Web/API/SyncManager)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct SyncManager {
    inner: Any,
}
impl FromVal for SyncManager {
    fn from_val(v: &Any) -> Self {
        SyncManager {
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
impl core::ops::Deref for SyncManager {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for SyncManager {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for SyncManager {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for SyncManager {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<SyncManager> for Any {
    fn from(s: SyncManager) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&SyncManager> for Any {
    fn from(s: &SyncManager) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(SyncManager);

impl SyncManager {
    /// The register method.
    /// [`SyncManager.register`](https://developer.mozilla.org/en-US/docs/Web/API/SyncManager/register)
    pub fn register(&self, tag: &str) -> Promise<Undefined> {
        self.inner
            .call("register", &[tag.into()])
            .as_::<Promise<Undefined>>()
    }
}
impl SyncManager {
    /// The getTags method.
    /// [`SyncManager.getTags`](https://developer.mozilla.org/en-US/docs/Web/API/SyncManager/getTags)
    pub fn get_tags(&self) -> Promise<Sequence<String>> {
        self.inner
            .call("getTags", &[])
            .as_::<Promise<Sequence<String>>>()
    }
}

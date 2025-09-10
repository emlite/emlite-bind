use super::*;

/// The LockManager class.
/// [`LockManager`](https://developer.mozilla.org/en-US/docs/Web/API/LockManager)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct LockManager {
    inner: Any,
}

impl FromVal for LockManager {
    fn from_val(v: &Any) -> Self {
        LockManager {
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

impl core::ops::Deref for LockManager {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for LockManager {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for LockManager {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for LockManager {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<LockManager> for Any {
    fn from(s: LockManager) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&LockManager> for Any {
    fn from(s: &LockManager) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(LockManager);

impl LockManager {
    /// The request method.
    /// [`LockManager.request`](https://developer.mozilla.org/en-US/docs/Web/API/LockManager/request)
    pub fn request(
        &self,
        name: &JsString,
        options: &LockOptions,
        callback: &Function,
    ) -> Promise<Any> {
        self.inner
            .call("request", &[name.into(), options.into(), callback.into()])
            .as_::<Promise<Any>>()
    }
}
impl LockManager {
    /// The query method.
    /// [`LockManager.query`](https://developer.mozilla.org/en-US/docs/Web/API/LockManager/query)
    pub fn query(&self) -> Promise<LockManagerSnapshot> {
        self.inner
            .call("query", &[])
            .as_::<Promise<LockManagerSnapshot>>()
    }
}

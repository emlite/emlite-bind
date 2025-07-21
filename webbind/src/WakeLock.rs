use super::*;

/// The WakeLock class.
/// [`WakeLock`](https://developer.mozilla.org/en-US/docs/Web/API/WakeLock)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct WakeLock {
    inner: Any,
}
impl FromVal for WakeLock {
    fn from_val(v: &Any) -> Self {
        WakeLock {
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
impl core::ops::Deref for WakeLock {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for WakeLock {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for WakeLock {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for WakeLock {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<WakeLock> for Any {
    fn from(s: WakeLock) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&WakeLock> for Any {
    fn from(s: &WakeLock) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(WakeLock);

impl WakeLock {
    /// The request method.
    /// [`WakeLock.request`](https://developer.mozilla.org/en-US/docs/Web/API/WakeLock/request)
    pub fn request0(&self) -> Promise<WakeLockSentinel> {
        self.inner
            .call("request", &[])
            .as_::<Promise<WakeLockSentinel>>()
    }
    /// The request method.
    /// [`WakeLock.request`](https://developer.mozilla.org/en-US/docs/Web/API/WakeLock/request)
    pub fn request1(&self, type_: &WakeLockType) -> Promise<WakeLockSentinel> {
        self.inner
            .call("request", &[type_.into()])
            .as_::<Promise<WakeLockSentinel>>()
    }
}

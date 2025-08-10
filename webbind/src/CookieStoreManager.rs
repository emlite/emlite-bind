use super::*;

/// The CookieStoreManager class.
/// [`CookieStoreManager`](https://developer.mozilla.org/en-US/docs/Web/API/CookieStoreManager)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct CookieStoreManager {
    inner: Any,
}

impl FromVal for CookieStoreManager {
    fn from_val(v: &Any) -> Self {
        CookieStoreManager {
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

impl core::ops::Deref for CookieStoreManager {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for CookieStoreManager {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for CookieStoreManager {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for CookieStoreManager {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<CookieStoreManager> for Any {
    fn from(s: CookieStoreManager) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&CookieStoreManager> for Any {
    fn from(s: &CookieStoreManager) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(CookieStoreManager);

impl CookieStoreManager {
    /// The subscribe method.
    /// [`CookieStoreManager.subscribe`](https://developer.mozilla.org/en-US/docs/Web/API/CookieStoreManager/subscribe)
    pub fn subscribe(
        &self,
        subscriptions: &TypedArray<CookieStoreGetOptions>,
    ) -> Promise<Undefined> {
        self.inner
            .call("subscribe", &[subscriptions.into()])
            .as_::<Promise<Undefined>>()
    }
}
impl CookieStoreManager {
    /// The getSubscriptions method.
    /// [`CookieStoreManager.getSubscriptions`](https://developer.mozilla.org/en-US/docs/Web/API/CookieStoreManager/getSubscriptions)
    pub fn get_subscriptions(&self) -> Promise<TypedArray<CookieStoreGetOptions>> {
        self.inner
            .call("getSubscriptions", &[])
            .as_::<Promise<TypedArray<CookieStoreGetOptions>>>()
    }
}
impl CookieStoreManager {
    /// The unsubscribe method.
    /// [`CookieStoreManager.unsubscribe`](https://developer.mozilla.org/en-US/docs/Web/API/CookieStoreManager/unsubscribe)
    pub fn unsubscribe(
        &self,
        subscriptions: &TypedArray<CookieStoreGetOptions>,
    ) -> Promise<Undefined> {
        self.inner
            .call("unsubscribe", &[subscriptions.into()])
            .as_::<Promise<Undefined>>()
    }
}

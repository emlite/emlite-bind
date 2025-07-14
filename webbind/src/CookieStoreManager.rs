use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct CookieStoreManager {
    inner: emlite::Val,
}
impl FromVal for CookieStoreManager {
    fn from_val(v: &emlite::Val) -> Self {
        CookieStoreManager {
            inner: emlite::Val::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for CookieStoreManager {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for CookieStoreManager {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for CookieStoreManager {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for CookieStoreManager {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<CookieStoreManager> for emlite::Val {
    fn from(s: CookieStoreManager) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(CookieStoreManager);

impl CookieStoreManager {
    pub fn subscribe(
        &self,
        subscriptions: jsbind::Sequence<CookieStoreGetOptions>,
    ) -> jsbind::Promise {
        self.inner
            .call("subscribe", &[subscriptions.into()])
            .as_::<jsbind::Promise>()
    }
}
impl CookieStoreManager {
    pub fn get_subscriptions(&self) -> jsbind::Promise {
        self.inner
            .call("getSubscriptions", &[])
            .as_::<jsbind::Promise>()
    }
}
impl CookieStoreManager {
    pub fn unsubscribe(
        &self,
        subscriptions: jsbind::Sequence<CookieStoreGetOptions>,
    ) -> jsbind::Promise {
        self.inner
            .call("unsubscribe", &[subscriptions.into()])
            .as_::<jsbind::Promise>()
    }
}

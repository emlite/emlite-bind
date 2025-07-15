use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct PushSubscriptionOptionsInit {
    inner: emlite::Val,
}
impl FromVal for PushSubscriptionOptionsInit {
    fn from_val(v: &emlite::Val) -> Self {
        PushSubscriptionOptionsInit { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for PushSubscriptionOptionsInit {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for PushSubscriptionOptionsInit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for PushSubscriptionOptionsInit {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for PushSubscriptionOptionsInit {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<PushSubscriptionOptionsInit> for emlite::Val {
    fn from(s: PushSubscriptionOptionsInit) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&PushSubscriptionOptionsInit> for emlite::Val {
    fn from(s: &PushSubscriptionOptionsInit) -> emlite::Val {
        s.inner.clone()
    }
}

impl PushSubscriptionOptionsInit {
    pub fn user_visible_only(&self) -> bool {
        self.inner.get("userVisibleOnly").as_::<bool>()
    }

    pub fn set_user_visible_only(&mut self, value: bool) {
        self.inner.set("userVisibleOnly", value);
    }
}
impl PushSubscriptionOptionsInit {
    pub fn application_server_key(&self) -> Any {
        self.inner.get("applicationServerKey").as_::<Any>()
    }

    pub fn set_application_server_key(&mut self, value: Any) {
        self.inner.set("applicationServerKey", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct PushManager {
    inner: emlite::Val,
}
impl FromVal for PushManager {
    fn from_val(v: &emlite::Val) -> Self {
        PushManager {
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
impl core::ops::Deref for PushManager {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for PushManager {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for PushManager {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for PushManager {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<PushManager> for emlite::Val {
    fn from(s: PushManager) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&PushManager> for emlite::Val {
    fn from(s: &PushManager) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(PushManager);

impl PushManager {
    pub fn supported_content_encodings() -> FrozenArray<DOMString> {
        emlite::Val::global("PushManager")
            .get("supportedContentEncodings")
            .as_::<FrozenArray<DOMString>>()
    }
}
impl PushManager {
    pub fn subscribe0(&self) -> Promise {
        self.inner.call("subscribe", &[]).as_::<Promise>()
    }

    pub fn subscribe1(&self, options: PushSubscriptionOptionsInit) -> Promise {
        self.inner
            .call("subscribe", &[options.into()])
            .as_::<Promise>()
    }
}
impl PushManager {
    pub fn get_subscription(&self) -> Promise {
        self.inner.call("getSubscription", &[]).as_::<Promise>()
    }
}
impl PushManager {
    pub fn permission_state0(&self) -> Promise {
        self.inner.call("permissionState", &[]).as_::<Promise>()
    }

    pub fn permission_state1(&self, options: PushSubscriptionOptionsInit) -> Promise {
        self.inner
            .call("permissionState", &[options.into()])
            .as_::<Promise>()
    }
}

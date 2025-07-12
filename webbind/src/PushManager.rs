use super::*;

#[derive(Clone, Debug)]
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
impl std::ops::Deref for PushSubscriptionOptionsInit {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for PushSubscriptionOptionsInit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<PushSubscriptionOptionsInit> for emlite::Val {
    fn from(s: PushSubscriptionOptionsInit) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
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
    pub fn application_server_key(&self) -> jsbind::Any {
        self.inner.get("applicationServerKey").as_::<jsbind::Any>()
    }

    pub fn set_application_server_key(&mut self, value: jsbind::Any) {
        self.inner.set("applicationServerKey", value);
    }
}
#[derive(Clone, Debug)]
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
impl std::ops::Deref for PushManager {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for PushManager {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<PushManager> for emlite::Val {
    fn from(s: PushManager) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl PushManager {
    pub fn supported_content_encodings() -> jsbind::FrozenArray<jsbind::DOMString> {
        emlite::Val::global("pushmanager")
            .get("supportedContentEncodings")
            .as_::<jsbind::FrozenArray<jsbind::DOMString>>()
    }
}
impl PushManager {
    pub fn subscribe0(&self) -> jsbind::Promise {
        self.inner.call("subscribe", &[]).as_::<jsbind::Promise>()
    }

    pub fn subscribe1(&self, options: PushSubscriptionOptionsInit) -> jsbind::Promise {
        self.inner
            .call("subscribe", &[options.into()])
            .as_::<jsbind::Promise>()
    }
}
impl PushManager {
    pub fn get_subscription(&self) -> jsbind::Promise {
        self.inner
            .call("getSubscription", &[])
            .as_::<jsbind::Promise>()
    }
}
impl PushManager {
    pub fn permission_state0(&self) -> jsbind::Promise {
        self.inner
            .call("permissionState", &[])
            .as_::<jsbind::Promise>()
    }

    pub fn permission_state1(&self, options: PushSubscriptionOptionsInit) -> jsbind::Promise {
        self.inner
            .call("permissionState", &[options.into()])
            .as_::<jsbind::Promise>()
    }
}

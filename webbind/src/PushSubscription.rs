use super::*;

#[derive(Clone, Debug)]
pub struct PushSubscriptionJSON {
    inner: emlite::Val,
}
impl FromVal for PushSubscriptionJSON {
    fn from_val(v: &emlite::Val) -> Self {
        PushSubscriptionJSON { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for PushSubscriptionJSON {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for PushSubscriptionJSON {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<PushSubscriptionJSON> for emlite::Val {
    fn from(s: PushSubscriptionJSON) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl PushSubscriptionJSON {
    pub fn endpoint(&self) -> jsbind::USVString {
        self.inner.get("endpoint").as_::<jsbind::USVString>()
    }

    pub fn set_endpoint(&mut self, value: jsbind::USVString) {
        self.inner.set("endpoint", value);
    }
}
impl PushSubscriptionJSON {
    pub fn expiration_time(&self) -> jsbind::Any {
        self.inner.get("expirationTime").as_::<jsbind::Any>()
    }

    pub fn set_expiration_time(&mut self, value: jsbind::Any) {
        self.inner.set("expirationTime", value);
    }
}
impl PushSubscriptionJSON {
    pub fn keys(&self) -> jsbind::Record<jsbind::DOMString, jsbind::USVString> {
        self.inner
            .get("keys")
            .as_::<jsbind::Record<jsbind::DOMString, jsbind::USVString>>()
    }

    pub fn set_keys(&mut self, value: jsbind::Record<jsbind::DOMString, jsbind::USVString>) {
        self.inner.set("keys", value);
    }
}
#[derive(Clone, Debug)]
pub struct PushSubscription {
    inner: emlite::Val,
}
impl FromVal for PushSubscription {
    fn from_val(v: &emlite::Val) -> Self {
        PushSubscription {
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
impl std::ops::Deref for PushSubscription {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for PushSubscription {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<PushSubscription> for emlite::Val {
    fn from(s: PushSubscription) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl PushSubscription {
    pub fn endpoint(&self) -> jsbind::USVString {
        self.inner.get("endpoint").as_::<jsbind::USVString>()
    }
}
impl PushSubscription {
    pub fn expiration_time(&self) -> jsbind::Any {
        self.inner.get("expirationTime").as_::<jsbind::Any>()
    }
}
impl PushSubscription {
    pub fn options(&self) -> PushSubscriptionOptions {
        self.inner.get("options").as_::<PushSubscriptionOptions>()
    }
}
impl PushSubscription {
    pub fn get_key(&self, name: PushEncryptionKeyName) -> jsbind::ArrayBuffer {
        self.inner
            .call("getKey", &[name.into()])
            .as_::<jsbind::ArrayBuffer>()
    }
}
impl PushSubscription {
    pub fn unsubscribe(&self) -> jsbind::Promise {
        self.inner.call("unsubscribe", &[]).as_::<jsbind::Promise>()
    }
}
impl PushSubscription {
    pub fn to_json(&self) -> PushSubscriptionJSON {
        self.inner.call("toJSON", &[]).as_::<PushSubscriptionJSON>()
    }
}

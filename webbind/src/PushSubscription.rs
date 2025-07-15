use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
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
impl core::ops::Deref for PushSubscriptionJSON {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for PushSubscriptionJSON {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for PushSubscriptionJSON {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for PushSubscriptionJSON {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<PushSubscriptionJSON> for emlite::Val {
    fn from(s: PushSubscriptionJSON) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&PushSubscriptionJSON> for emlite::Val {
    fn from(s: &PushSubscriptionJSON) -> emlite::Val {
        s.inner.clone()
    }
}

impl PushSubscriptionJSON {
    pub fn endpoint(&self) -> USVString {
        self.inner.get("endpoint").as_::<USVString>()
    }

    pub fn set_endpoint(&mut self, value: USVString) {
        self.inner.set("endpoint", value);
    }
}
impl PushSubscriptionJSON {
    pub fn expiration_time(&self) -> Any {
        self.inner.get("expirationTime").as_::<Any>()
    }

    pub fn set_expiration_time(&mut self, value: Any) {
        self.inner.set("expirationTime", value);
    }
}
impl PushSubscriptionJSON {
    pub fn keys(&self) -> Record<DOMString, USVString> {
        self.inner.get("keys").as_::<Record<DOMString, USVString>>()
    }

    pub fn set_keys(&mut self, value: Record<DOMString, USVString>) {
        self.inner.set("keys", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
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
impl core::ops::Deref for PushSubscription {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for PushSubscription {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for PushSubscription {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for PushSubscription {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<PushSubscription> for emlite::Val {
    fn from(s: PushSubscription) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&PushSubscription> for emlite::Val {
    fn from(s: &PushSubscription) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(PushSubscription);

impl PushSubscription {
    pub fn endpoint(&self) -> USVString {
        self.inner.get("endpoint").as_::<USVString>()
    }
}
impl PushSubscription {
    pub fn expiration_time(&self) -> Any {
        self.inner.get("expirationTime").as_::<Any>()
    }
}
impl PushSubscription {
    pub fn options(&self) -> PushSubscriptionOptions {
        self.inner.get("options").as_::<PushSubscriptionOptions>()
    }
}
impl PushSubscription {
    pub fn get_key(&self, name: PushEncryptionKeyName) -> ArrayBuffer {
        self.inner
            .call("getKey", &[name.into()])
            .as_::<ArrayBuffer>()
    }
}
impl PushSubscription {
    pub fn unsubscribe(&self) -> Promise {
        self.inner.call("unsubscribe", &[]).as_::<Promise>()
    }
}
impl PushSubscription {
    pub fn to_json(&self) -> PushSubscriptionJSON {
        self.inner.call("toJSON", &[]).as_::<PushSubscriptionJSON>()
    }
}

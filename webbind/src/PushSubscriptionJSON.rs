use super::*;

/// The PushSubscriptionJSON dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct PushSubscriptionJSON {
    inner: Any,
}

impl FromVal for PushSubscriptionJSON {
    fn from_val(v: &Any) -> Self {
        PushSubscriptionJSON { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for PushSubscriptionJSON {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for PushSubscriptionJSON {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for PushSubscriptionJSON {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for PushSubscriptionJSON {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<PushSubscriptionJSON> for Any {
    fn from(s: PushSubscriptionJSON) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&PushSubscriptionJSON> for Any {
    fn from(s: &PushSubscriptionJSON) -> Any {
        s.inner.clone()
    }
}

impl PushSubscriptionJSON {
    /// Getter of the `endpoint` attribute.
    pub fn endpoint(&self) -> JsString {
        self.inner.get("endpoint").as_::<JsString>()
    }

    /// Setter of the `endpoint` attribute.
    pub fn set_endpoint(&mut self, value: &JsString) {
        self.inner.set("endpoint", value);
    }
}
impl PushSubscriptionJSON {
    /// Getter of the `expirationTime` attribute.
    pub fn expiration_time(&self) -> Any {
        self.inner.get("expirationTime").as_::<Any>()
    }

    /// Setter of the `expirationTime` attribute.
    pub fn set_expiration_time(&mut self, value: &Any) {
        self.inner.set("expirationTime", value);
    }
}
impl PushSubscriptionJSON {
    /// Getter of the `keys` attribute.
    pub fn keys(&self) -> Record<JsString, JsString> {
        self.inner.get("keys").as_::<Record<JsString, JsString>>()
    }

    /// Setter of the `keys` attribute.
    pub fn set_keys(&mut self, value: &Record<JsString, JsString>) {
        self.inner.set("keys", value);
    }
}

use super::*;

/// The PushSubscription class.
/// [`PushSubscription`](https://developer.mozilla.org/en-US/docs/Web/API/PushSubscription)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct PushSubscription {
    inner: Any,
}

impl FromVal for PushSubscription {
    fn from_val(v: &Any) -> Self {
        PushSubscription {
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

impl core::ops::Deref for PushSubscription {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for PushSubscription {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for PushSubscription {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for PushSubscription {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<PushSubscription> for Any {
    fn from(s: PushSubscription) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&PushSubscription> for Any {
    fn from(s: &PushSubscription) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(PushSubscription);

impl PushSubscription {
    /// Getter of the `endpoint` attribute.
    /// [`PushSubscription.endpoint`](https://developer.mozilla.org/en-US/docs/Web/API/PushSubscription/endpoint)
    pub fn endpoint(&self) -> JsString {
        self.inner.get("endpoint").as_::<JsString>()
    }
}
impl PushSubscription {
    /// Getter of the `expirationTime` attribute.
    /// [`PushSubscription.expirationTime`](https://developer.mozilla.org/en-US/docs/Web/API/PushSubscription/expirationTime)
    pub fn expiration_time(&self) -> Any {
        self.inner.get("expirationTime").as_::<Any>()
    }
}
impl PushSubscription {
    /// Getter of the `options` attribute.
    /// [`PushSubscription.options`](https://developer.mozilla.org/en-US/docs/Web/API/PushSubscription/options)
    pub fn options(&self) -> PushSubscriptionOptions {
        self.inner.get("options").as_::<PushSubscriptionOptions>()
    }
}
impl PushSubscription {
    /// The getKey method.
    /// [`PushSubscription.getKey`](https://developer.mozilla.org/en-US/docs/Web/API/PushSubscription/getKey)
    pub fn get_key(&self, name: &PushEncryptionKeyName) -> ArrayBuffer {
        self.inner
            .call("getKey", &[name.into()])
            .as_::<ArrayBuffer>()
    }
}
impl PushSubscription {
    /// The unsubscribe method.
    /// [`PushSubscription.unsubscribe`](https://developer.mozilla.org/en-US/docs/Web/API/PushSubscription/unsubscribe)
    pub fn unsubscribe(&self) -> Promise<bool> {
        self.inner.call("unsubscribe", &[]).as_::<Promise<bool>>()
    }
}
impl PushSubscription {
    /// The toJSON method.
    /// [`PushSubscription.toJSON`](https://developer.mozilla.org/en-US/docs/Web/API/PushSubscription/toJSON)
    pub fn to_json(&self) -> PushSubscriptionJSON {
        self.inner.call("toJSON", &[]).as_::<PushSubscriptionJSON>()
    }
}

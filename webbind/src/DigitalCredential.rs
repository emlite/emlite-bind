use super::*;

/// The DigitalCredential class.
/// [`DigitalCredential`](https://developer.mozilla.org/en-US/docs/Web/API/DigitalCredential)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct DigitalCredential {
    inner: Credential,
}
impl FromVal for DigitalCredential {
    fn from_val(v: &Any) -> Self {
        DigitalCredential {
            inner: Credential::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for DigitalCredential {
    type Target = Credential;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for DigitalCredential {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for DigitalCredential {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for DigitalCredential {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<DigitalCredential> for Any {
    fn from(s: DigitalCredential) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&DigitalCredential> for Any {
    fn from(s: &DigitalCredential) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(DigitalCredential);

impl DigitalCredential {
    /// The toJSON method.
    /// [`DigitalCredential.toJSON`](https://developer.mozilla.org/en-US/docs/Web/API/DigitalCredential/toJSON)
    pub fn to_json(&self) -> Object {
        self.inner.call("toJSON", &[]).as_::<Object>()
    }
}
impl DigitalCredential {
    /// Getter of the `protocol` attribute.
    /// [`DigitalCredential.protocol`](https://developer.mozilla.org/en-US/docs/Web/API/DigitalCredential/protocol)
    pub fn protocol(&self) -> JsString {
        self.inner.get("protocol").as_::<JsString>()
    }
}
impl DigitalCredential {
    /// Getter of the `data` attribute.
    /// [`DigitalCredential.data`](https://developer.mozilla.org/en-US/docs/Web/API/DigitalCredential/data)
    pub fn data(&self) -> Object {
        self.inner.get("data").as_::<Object>()
    }
}
impl DigitalCredential {
    /// The userAgentAllowsProtocol method.
    /// [`DigitalCredential.userAgentAllowsProtocol`](https://developer.mozilla.org/en-US/docs/Web/API/DigitalCredential/userAgentAllowsProtocol)
    pub fn user_agent_allows_protocol(protocol: &JsString) -> bool {
        Any::global("DigitalCredential")
            .call("userAgentAllowsProtocol", &[protocol.into()])
            .as_::<bool>()
    }
}

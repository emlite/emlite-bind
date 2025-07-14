use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct DigitalCredential {
    inner: Credential,
}
impl FromVal for DigitalCredential {
    fn from_val(v: &emlite::Val) -> Self {
        DigitalCredential {
            inner: Credential::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
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
impl From<DigitalCredential> for emlite::Val {
    fn from(s: DigitalCredential) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl DigitalCredential {
    pub fn to_json(&self) -> jsbind::Object {
        self.inner.call("toJSON", &[]).as_::<jsbind::Object>()
    }
}
impl DigitalCredential {
    pub fn protocol(&self) -> jsbind::DOMString {
        self.inner.get("protocol").as_::<jsbind::DOMString>()
    }
}
impl DigitalCredential {
    pub fn data(&self) -> jsbind::Object {
        self.inner.get("data").as_::<jsbind::Object>()
    }
}
impl DigitalCredential {
    pub fn user_agent_allows_protocol(protocol: jsbind::DOMString) -> bool {
        emlite::Val::global("digitalcredential")
            .call("userAgentAllowsProtocol", &[protocol.into()])
            .as_::<bool>()
    }
}

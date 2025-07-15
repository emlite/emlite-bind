use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
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
impl AsRef<emlite::Val> for DigitalCredential {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for DigitalCredential {
    fn as_mut(&mut self) -> &mut emlite::Val {
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
impl From<&DigitalCredential> for emlite::Val {
    fn from(s: &DigitalCredential) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(DigitalCredential);

impl DigitalCredential {
    pub fn to_json(&self) -> Object {
        self.inner.call("toJSON", &[]).as_::<Object>()
    }
}
impl DigitalCredential {
    pub fn protocol(&self) -> String {
        self.inner.get("protocol").as_::<String>()
    }
}
impl DigitalCredential {
    pub fn data(&self) -> Object {
        self.inner.get("data").as_::<Object>()
    }
}
impl DigitalCredential {
    pub fn user_agent_allows_protocol(protocol: &str) -> bool {
        emlite::Val::global("DigitalCredential")
            .call("userAgentAllowsProtocol", &[protocol.into()])
            .as_::<bool>()
    }
}

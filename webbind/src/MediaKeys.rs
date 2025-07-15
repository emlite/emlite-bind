use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct MediaKeysPolicy {
    inner: emlite::Val,
}
impl FromVal for MediaKeysPolicy {
    fn from_val(v: &emlite::Val) -> Self {
        MediaKeysPolicy { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for MediaKeysPolicy {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for MediaKeysPolicy {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for MediaKeysPolicy {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for MediaKeysPolicy {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<MediaKeysPolicy> for emlite::Val {
    fn from(s: MediaKeysPolicy) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&MediaKeysPolicy> for emlite::Val {
    fn from(s: &MediaKeysPolicy) -> emlite::Val {
        s.inner.clone()
    }
}

impl MediaKeysPolicy {
    pub fn min_hdcp_version(&self) -> DOMString {
        self.inner.get("minHdcpVersion").as_::<DOMString>()
    }

    pub fn set_min_hdcp_version(&mut self, value: DOMString) {
        self.inner.set("minHdcpVersion", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct MediaKeys {
    inner: emlite::Val,
}
impl FromVal for MediaKeys {
    fn from_val(v: &emlite::Val) -> Self {
        MediaKeys {
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
impl core::ops::Deref for MediaKeys {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for MediaKeys {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for MediaKeys {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for MediaKeys {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<MediaKeys> for emlite::Val {
    fn from(s: MediaKeys) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&MediaKeys> for emlite::Val {
    fn from(s: &MediaKeys) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(MediaKeys);

impl MediaKeys {
    pub fn create_session0(&self) -> MediaKeySession {
        self.inner
            .call("createSession", &[])
            .as_::<MediaKeySession>()
    }

    pub fn create_session1(&self, session_type: MediaKeySessionType) -> MediaKeySession {
        self.inner
            .call("createSession", &[session_type.into()])
            .as_::<MediaKeySession>()
    }
}
impl MediaKeys {
    pub fn get_status_for_policy0(&self) -> Promise {
        self.inner.call("getStatusForPolicy", &[]).as_::<Promise>()
    }

    pub fn get_status_for_policy1(&self, policy: MediaKeysPolicy) -> Promise {
        self.inner
            .call("getStatusForPolicy", &[policy.into()])
            .as_::<Promise>()
    }
}
impl MediaKeys {
    pub fn set_server_certificate(&self, server_certificate: Any) -> Promise {
        self.inner
            .call("setServerCertificate", &[server_certificate.into()])
            .as_::<Promise>()
    }
}

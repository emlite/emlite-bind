use super::*;

#[derive(Clone, Debug)]
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
impl std::ops::Deref for MediaKeysPolicy {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for MediaKeysPolicy {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<MediaKeysPolicy> for emlite::Val {
    fn from(s: MediaKeysPolicy) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl MediaKeysPolicy {
    pub fn min_hdcp_version(&self) -> jsbind::DOMString {
        self.inner.get("minHdcpVersion").as_::<jsbind::DOMString>()
    }

    pub fn set_min_hdcp_version(&mut self, value: jsbind::DOMString) {
        self.inner.set("minHdcpVersion", value);
    }
}
#[derive(Clone, Debug)]
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
impl std::ops::Deref for MediaKeys {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for MediaKeys {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<MediaKeys> for emlite::Val {
    fn from(s: MediaKeys) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

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
    pub fn get_status_for_policy0(&self) -> jsbind::Promise {
        self.inner
            .call("getStatusForPolicy", &[])
            .as_::<jsbind::Promise>()
    }

    pub fn get_status_for_policy1(&self, policy: MediaKeysPolicy) -> jsbind::Promise {
        self.inner
            .call("getStatusForPolicy", &[policy.into()])
            .as_::<jsbind::Promise>()
    }
}
impl MediaKeys {
    pub fn set_server_certificate(&self, server_certificate: jsbind::Any) -> jsbind::Promise {
        self.inner
            .call("setServerCertificate", &[server_certificate.into()])
            .as_::<jsbind::Promise>()
    }
}

use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct MediaKeysPolicy {
    inner: Any,
}
impl FromVal for MediaKeysPolicy {
    fn from_val(v: &Any) -> Self {
        MediaKeysPolicy { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for MediaKeysPolicy {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for MediaKeysPolicy {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for MediaKeysPolicy {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for MediaKeysPolicy {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<MediaKeysPolicy> for Any {
    fn from(s: MediaKeysPolicy) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&MediaKeysPolicy> for Any {
    fn from(s: &MediaKeysPolicy) -> Any {
        s.inner.clone()
    }
}

impl MediaKeysPolicy {
    pub fn min_hdcp_version(&self) -> String {
        self.inner.get("minHdcpVersion").as_::<String>()
    }

    pub fn set_min_hdcp_version(&mut self, value: &str) {
        self.inner.set("minHdcpVersion", value);
    }
}
/// The MediaKeys class.
/// [`MediaKeys`](https://developer.mozilla.org/en-US/docs/Web/API/MediaKeys)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct MediaKeys {
    inner: Any,
}
impl FromVal for MediaKeys {
    fn from_val(v: &Any) -> Self {
        MediaKeys {
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
impl core::ops::Deref for MediaKeys {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for MediaKeys {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for MediaKeys {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for MediaKeys {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<MediaKeys> for Any {
    fn from(s: MediaKeys) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&MediaKeys> for Any {
    fn from(s: &MediaKeys) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(MediaKeys);

impl MediaKeys {
    /// The createSession method.
    /// [`MediaKeys.createSession`](https://developer.mozilla.org/en-US/docs/Web/API/MediaKeys/createSession)
    pub fn create_session0(&self) -> MediaKeySession {
        self.inner
            .call("createSession", &[])
            .as_::<MediaKeySession>()
    }
    /// The createSession method.
    /// [`MediaKeys.createSession`](https://developer.mozilla.org/en-US/docs/Web/API/MediaKeys/createSession)
    pub fn create_session1(&self, session_type: &MediaKeySessionType) -> MediaKeySession {
        self.inner
            .call("createSession", &[session_type.into()])
            .as_::<MediaKeySession>()
    }
}
impl MediaKeys {
    /// The getStatusForPolicy method.
    /// [`MediaKeys.getStatusForPolicy`](https://developer.mozilla.org/en-US/docs/Web/API/MediaKeys/getStatusForPolicy)
    pub fn get_status_for_policy0(&self) -> Promise {
        self.inner.call("getStatusForPolicy", &[]).as_::<Promise>()
    }
    /// The getStatusForPolicy method.
    /// [`MediaKeys.getStatusForPolicy`](https://developer.mozilla.org/en-US/docs/Web/API/MediaKeys/getStatusForPolicy)
    pub fn get_status_for_policy1(&self, policy: &MediaKeysPolicy) -> Promise {
        self.inner
            .call("getStatusForPolicy", &[policy.into()])
            .as_::<Promise>()
    }
}
impl MediaKeys {
    /// The setServerCertificate method.
    /// [`MediaKeys.setServerCertificate`](https://developer.mozilla.org/en-US/docs/Web/API/MediaKeys/setServerCertificate)
    pub fn set_server_certificate(&self, server_certificate: &Any) -> Promise {
        self.inner
            .call("setServerCertificate", &[server_certificate.into()])
            .as_::<Promise>()
    }
}

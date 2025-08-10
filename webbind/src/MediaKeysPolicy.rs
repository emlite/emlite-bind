use super::*;

/// The MediaKeysPolicy dictionary.
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
    /// Getter of the `minHdcpVersion` attribute.
    pub fn min_hdcp_version(&self) -> JsString {
        self.inner.get("minHdcpVersion").as_::<JsString>()
    }

    /// Setter of the `minHdcpVersion` attribute.
    pub fn set_min_hdcp_version(&mut self, value: &JsString) {
        self.inner.set("minHdcpVersion", value);
    }
}

use super::*;




/// The WebTransportReceiveStreamStats dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct WebTransportReceiveStreamStats {
    inner: Any,
}

impl FromVal for WebTransportReceiveStreamStats {
    fn from_val(v: &Any) -> Self {
        WebTransportReceiveStreamStats { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for WebTransportReceiveStreamStats {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for WebTransportReceiveStreamStats {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for WebTransportReceiveStreamStats {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for WebTransportReceiveStreamStats {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<WebTransportReceiveStreamStats> for Any {
    fn from(s: WebTransportReceiveStreamStats) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&WebTransportReceiveStreamStats> for Any {
    fn from(s: &WebTransportReceiveStreamStats) -> Any {
        s.inner.clone()
    }
}

impl WebTransportReceiveStreamStats {
    /// Getter of the `bytesReceived` attribute.
    pub fn bytes_received(&self) -> u64 {
        self.inner.get("bytesReceived").as_::<u64>()
    }

    /// Setter of the `bytesReceived` attribute.
    pub fn set_bytes_received(&mut self, value: u64) {
        self.inner.set("bytesReceived", value);
    }
}
impl WebTransportReceiveStreamStats {
    /// Getter of the `bytesRead` attribute.
    pub fn bytes_read(&self) -> u64 {
        self.inner.get("bytesRead").as_::<u64>()
    }

    /// Setter of the `bytesRead` attribute.
    pub fn set_bytes_read(&mut self, value: u64) {
        self.inner.set("bytesRead", value);
    }
}

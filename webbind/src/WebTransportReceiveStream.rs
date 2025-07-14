use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct WebTransportReceiveStreamStats {
    inner: emlite::Val,
}
impl FromVal for WebTransportReceiveStreamStats {
    fn from_val(v: &emlite::Val) -> Self {
        WebTransportReceiveStreamStats { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for WebTransportReceiveStreamStats {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for WebTransportReceiveStreamStats {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<WebTransportReceiveStreamStats> for emlite::Val {
    fn from(s: WebTransportReceiveStreamStats) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl WebTransportReceiveStreamStats {
    pub fn bytes_received(&self) -> u64 {
        self.inner.get("bytesReceived").as_::<u64>()
    }

    pub fn set_bytes_received(&mut self, value: u64) {
        self.inner.set("bytesReceived", value);
    }
}
impl WebTransportReceiveStreamStats {
    pub fn bytes_read(&self) -> u64 {
        self.inner.get("bytesRead").as_::<u64>()
    }

    pub fn set_bytes_read(&mut self, value: u64) {
        self.inner.set("bytesRead", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct WebTransportReceiveStream {
    inner: ReadableStream,
}
impl FromVal for WebTransportReceiveStream {
    fn from_val(v: &emlite::Val) -> Self {
        WebTransportReceiveStream {
            inner: ReadableStream::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for WebTransportReceiveStream {
    type Target = ReadableStream;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for WebTransportReceiveStream {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<WebTransportReceiveStream> for emlite::Val {
    fn from(s: WebTransportReceiveStream) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl WebTransportReceiveStream {
    pub fn get_stats(&self) -> jsbind::Promise {
        self.inner.call("getStats", &[]).as_::<jsbind::Promise>()
    }
}

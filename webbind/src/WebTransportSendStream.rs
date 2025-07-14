use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct WebTransportSendStreamStats {
    inner: emlite::Val,
}
impl FromVal for WebTransportSendStreamStats {
    fn from_val(v: &emlite::Val) -> Self {
        WebTransportSendStreamStats { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for WebTransportSendStreamStats {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for WebTransportSendStreamStats {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<WebTransportSendStreamStats> for emlite::Val {
    fn from(s: WebTransportSendStreamStats) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl WebTransportSendStreamStats {
    pub fn bytes_written(&self) -> u64 {
        self.inner.get("bytesWritten").as_::<u64>()
    }

    pub fn set_bytes_written(&mut self, value: u64) {
        self.inner.set("bytesWritten", value);
    }
}
impl WebTransportSendStreamStats {
    pub fn bytes_sent(&self) -> u64 {
        self.inner.get("bytesSent").as_::<u64>()
    }

    pub fn set_bytes_sent(&mut self, value: u64) {
        self.inner.set("bytesSent", value);
    }
}
impl WebTransportSendStreamStats {
    pub fn bytes_acknowledged(&self) -> u64 {
        self.inner.get("bytesAcknowledged").as_::<u64>()
    }

    pub fn set_bytes_acknowledged(&mut self, value: u64) {
        self.inner.set("bytesAcknowledged", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct WebTransportSendStream {
    inner: WritableStream,
}
impl FromVal for WebTransportSendStream {
    fn from_val(v: &emlite::Val) -> Self {
        WebTransportSendStream {
            inner: WritableStream::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for WebTransportSendStream {
    type Target = WritableStream;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for WebTransportSendStream {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<WebTransportSendStream> for emlite::Val {
    fn from(s: WebTransportSendStream) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl WebTransportSendStream {
    pub fn send_group(&self) -> WebTransportSendGroup {
        self.inner.get("sendGroup").as_::<WebTransportSendGroup>()
    }

    pub fn set_send_group(&mut self, value: WebTransportSendGroup) {
        self.inner.set("sendGroup", value);
    }
}
impl WebTransportSendStream {
    pub fn send_order(&self) -> i64 {
        self.inner.get("sendOrder").as_::<i64>()
    }

    pub fn set_send_order(&mut self, value: i64) {
        self.inner.set("sendOrder", value);
    }
}
impl WebTransportSendStream {
    pub fn get_stats(&self) -> jsbind::Promise {
        self.inner.call("getStats", &[]).as_::<jsbind::Promise>()
    }
}
impl WebTransportSendStream {
    pub fn get_writer(&self) -> WebTransportWriter {
        self.inner
            .call("getWriter", &[])
            .as_::<WebTransportWriter>()
    }
}

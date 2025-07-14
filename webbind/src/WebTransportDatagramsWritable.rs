use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct WebTransportDatagramsWritable {
    inner: WritableStream,
}
impl FromVal for WebTransportDatagramsWritable {
    fn from_val(v: &emlite::Val) -> Self {
        WebTransportDatagramsWritable {
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
impl core::ops::Deref for WebTransportDatagramsWritable {
    type Target = WritableStream;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for WebTransportDatagramsWritable {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<WebTransportDatagramsWritable> for emlite::Val {
    fn from(s: WebTransportDatagramsWritable) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl WebTransportDatagramsWritable {
    pub fn send_group(&self) -> WebTransportSendGroup {
        self.inner.get("sendGroup").as_::<WebTransportSendGroup>()
    }

    pub fn set_send_group(&mut self, value: WebTransportSendGroup) {
        self.inner.set("sendGroup", value);
    }
}
impl WebTransportDatagramsWritable {
    pub fn send_order(&self) -> i64 {
        self.inner.get("sendOrder").as_::<i64>()
    }

    pub fn set_send_order(&mut self, value: i64) {
        self.inner.set("sendOrder", value);
    }
}

use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct WebTransportSendOptions {
    inner: emlite::Val,
}
impl FromVal for WebTransportSendOptions {
    fn from_val(v: &emlite::Val) -> Self {
        WebTransportSendOptions { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for WebTransportSendOptions {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for WebTransportSendOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for WebTransportSendOptions {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for WebTransportSendOptions {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<WebTransportSendOptions> for emlite::Val {
    fn from(s: WebTransportSendOptions) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&WebTransportSendOptions> for emlite::Val {
    fn from(s: &WebTransportSendOptions) -> emlite::Val {
        s.inner.clone()
    }
}

impl WebTransportSendOptions {
    pub fn send_group(&self) -> WebTransportSendGroup {
        self.inner.get("sendGroup").as_::<WebTransportSendGroup>()
    }

    pub fn set_send_group(&mut self, value: &WebTransportSendGroup) {
        self.inner.set("sendGroup", value);
    }
}
impl WebTransportSendOptions {
    pub fn send_order(&self) -> i64 {
        self.inner.get("sendOrder").as_::<i64>()
    }

    pub fn set_send_order(&mut self, value: i64) {
        self.inner.set("sendOrder", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct WebTransportDatagramDuplexStream {
    inner: emlite::Val,
}
impl FromVal for WebTransportDatagramDuplexStream {
    fn from_val(v: &emlite::Val) -> Self {
        WebTransportDatagramDuplexStream {
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
impl core::ops::Deref for WebTransportDatagramDuplexStream {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for WebTransportDatagramDuplexStream {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for WebTransportDatagramDuplexStream {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for WebTransportDatagramDuplexStream {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<WebTransportDatagramDuplexStream> for emlite::Val {
    fn from(s: WebTransportDatagramDuplexStream) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&WebTransportDatagramDuplexStream> for emlite::Val {
    fn from(s: &WebTransportDatagramDuplexStream) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(WebTransportDatagramDuplexStream);

impl WebTransportDatagramDuplexStream {
    pub fn create_writable0(&self) -> WebTransportDatagramsWritable {
        self.inner
            .call("createWritable", &[])
            .as_::<WebTransportDatagramsWritable>()
    }

    pub fn create_writable1(
        &self,
        options: &WebTransportSendOptions,
    ) -> WebTransportDatagramsWritable {
        self.inner
            .call("createWritable", &[options.into()])
            .as_::<WebTransportDatagramsWritable>()
    }
}
impl WebTransportDatagramDuplexStream {
    pub fn readable(&self) -> ReadableStream {
        self.inner.get("readable").as_::<ReadableStream>()
    }
}
impl WebTransportDatagramDuplexStream {
    pub fn max_datagram_size(&self) -> u32 {
        self.inner.get("maxDatagramSize").as_::<u32>()
    }
}
impl WebTransportDatagramDuplexStream {
    pub fn incoming_max_age(&self) -> f64 {
        self.inner.get("incomingMaxAge").as_::<f64>()
    }

    pub fn set_incoming_max_age(&mut self, value: f64) {
        self.inner.set("incomingMaxAge", value);
    }
}
impl WebTransportDatagramDuplexStream {
    pub fn outgoing_max_age(&self) -> f64 {
        self.inner.get("outgoingMaxAge").as_::<f64>()
    }

    pub fn set_outgoing_max_age(&mut self, value: f64) {
        self.inner.set("outgoingMaxAge", value);
    }
}
impl WebTransportDatagramDuplexStream {
    pub fn incoming_high_water_mark(&self) -> f64 {
        self.inner.get("incomingHighWaterMark").as_::<f64>()
    }

    pub fn set_incoming_high_water_mark(&mut self, value: f64) {
        self.inner.set("incomingHighWaterMark", value);
    }
}
impl WebTransportDatagramDuplexStream {
    pub fn outgoing_high_water_mark(&self) -> f64 {
        self.inner.get("outgoingHighWaterMark").as_::<f64>()
    }

    pub fn set_outgoing_high_water_mark(&mut self, value: f64) {
        self.inner.set("outgoingHighWaterMark", value);
    }
}

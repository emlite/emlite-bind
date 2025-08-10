use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct RTCDataChannelStats {
    inner: Any,
}
impl FromVal for RTCDataChannelStats {
    fn from_val(v: &Any) -> Self {
        RTCDataChannelStats { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for RTCDataChannelStats {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for RTCDataChannelStats {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for RTCDataChannelStats {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for RTCDataChannelStats {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<RTCDataChannelStats> for Any {
    fn from(s: RTCDataChannelStats) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&RTCDataChannelStats> for Any {
    fn from(s: &RTCDataChannelStats) -> Any {
        s.inner.clone()
    }
}

impl RTCDataChannelStats {
    pub fn label(&self) -> JsString {
        self.inner.get("label").as_::<JsString>()
    }

    pub fn set_label(&mut self, value: &JsString) {
        self.inner.set("label", value);
    }
}
impl RTCDataChannelStats {
    pub fn protocol(&self) -> JsString {
        self.inner.get("protocol").as_::<JsString>()
    }

    pub fn set_protocol(&mut self, value: &JsString) {
        self.inner.set("protocol", value);
    }
}
impl RTCDataChannelStats {
    pub fn data_channel_identifier(&self) -> u16 {
        self.inner.get("dataChannelIdentifier").as_::<u16>()
    }

    pub fn set_data_channel_identifier(&mut self, value: u16) {
        self.inner.set("dataChannelIdentifier", value);
    }
}
impl RTCDataChannelStats {
    pub fn state(&self) -> RTCDataChannelState {
        self.inner.get("state").as_::<RTCDataChannelState>()
    }

    pub fn set_state(&mut self, value: &RTCDataChannelState) {
        self.inner.set("state", value);
    }
}
impl RTCDataChannelStats {
    pub fn messages_sent(&self) -> u32 {
        self.inner.get("messagesSent").as_::<u32>()
    }

    pub fn set_messages_sent(&mut self, value: u32) {
        self.inner.set("messagesSent", value);
    }
}
impl RTCDataChannelStats {
    pub fn bytes_sent(&self) -> u64 {
        self.inner.get("bytesSent").as_::<u64>()
    }

    pub fn set_bytes_sent(&mut self, value: u64) {
        self.inner.set("bytesSent", value);
    }
}
impl RTCDataChannelStats {
    pub fn messages_received(&self) -> u32 {
        self.inner.get("messagesReceived").as_::<u32>()
    }

    pub fn set_messages_received(&mut self, value: u32) {
        self.inner.set("messagesReceived", value);
    }
}
impl RTCDataChannelStats {
    pub fn bytes_received(&self) -> u64 {
        self.inner.get("bytesReceived").as_::<u64>()
    }

    pub fn set_bytes_received(&mut self, value: u64) {
        self.inner.set("bytesReceived", value);
    }
}

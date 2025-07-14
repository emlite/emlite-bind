use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct RTCDataChannel {
    inner: EventTarget,
}
impl FromVal for RTCDataChannel {
    fn from_val(v: &emlite::Val) -> Self {
        RTCDataChannel {
            inner: EventTarget::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for RTCDataChannel {
    type Target = EventTarget;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for RTCDataChannel {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for RTCDataChannel {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for RTCDataChannel {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<RTCDataChannel> for emlite::Val {
    fn from(s: RTCDataChannel) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(RTCDataChannel);

impl RTCDataChannel {
    pub fn label(&self) -> jsbind::USVString {
        self.inner.get("label").as_::<jsbind::USVString>()
    }
}
impl RTCDataChannel {
    pub fn ordered(&self) -> bool {
        self.inner.get("ordered").as_::<bool>()
    }
}
impl RTCDataChannel {
    pub fn max_packet_life_time(&self) -> u16 {
        self.inner.get("maxPacketLifeTime").as_::<u16>()
    }
}
impl RTCDataChannel {
    pub fn max_retransmits(&self) -> u16 {
        self.inner.get("maxRetransmits").as_::<u16>()
    }
}
impl RTCDataChannel {
    pub fn protocol(&self) -> jsbind::USVString {
        self.inner.get("protocol").as_::<jsbind::USVString>()
    }
}
impl RTCDataChannel {
    pub fn negotiated(&self) -> bool {
        self.inner.get("negotiated").as_::<bool>()
    }
}
impl RTCDataChannel {
    pub fn id(&self) -> u16 {
        self.inner.get("id").as_::<u16>()
    }
}
impl RTCDataChannel {
    pub fn ready_state(&self) -> RTCDataChannelState {
        self.inner.get("readyState").as_::<RTCDataChannelState>()
    }
}
impl RTCDataChannel {
    pub fn buffered_amount(&self) -> u32 {
        self.inner.get("bufferedAmount").as_::<u32>()
    }
}
impl RTCDataChannel {
    pub fn buffered_amount_low_threshold(&self) -> u32 {
        self.inner.get("bufferedAmountLowThreshold").as_::<u32>()
    }

    pub fn set_buffered_amount_low_threshold(&mut self, value: u32) {
        self.inner.set("bufferedAmountLowThreshold", value);
    }
}
impl RTCDataChannel {
    pub fn onopen(&self) -> jsbind::Any {
        self.inner.get("onopen").as_::<jsbind::Any>()
    }

    pub fn set_onopen(&mut self, value: jsbind::Any) {
        self.inner.set("onopen", value);
    }
}
impl RTCDataChannel {
    pub fn onbufferedamountlow(&self) -> jsbind::Any {
        self.inner.get("onbufferedamountlow").as_::<jsbind::Any>()
    }

    pub fn set_onbufferedamountlow(&mut self, value: jsbind::Any) {
        self.inner.set("onbufferedamountlow", value);
    }
}
impl RTCDataChannel {
    pub fn onerror(&self) -> jsbind::Any {
        self.inner.get("onerror").as_::<jsbind::Any>()
    }

    pub fn set_onerror(&mut self, value: jsbind::Any) {
        self.inner.set("onerror", value);
    }
}
impl RTCDataChannel {
    pub fn onclosing(&self) -> jsbind::Any {
        self.inner.get("onclosing").as_::<jsbind::Any>()
    }

    pub fn set_onclosing(&mut self, value: jsbind::Any) {
        self.inner.set("onclosing", value);
    }
}
impl RTCDataChannel {
    pub fn onclose(&self) -> jsbind::Any {
        self.inner.get("onclose").as_::<jsbind::Any>()
    }

    pub fn set_onclose(&mut self, value: jsbind::Any) {
        self.inner.set("onclose", value);
    }
}
impl RTCDataChannel {
    pub fn close(&self) -> jsbind::Undefined {
        self.inner.call("close", &[]).as_::<jsbind::Undefined>()
    }
}
impl RTCDataChannel {
    pub fn onmessage(&self) -> jsbind::Any {
        self.inner.get("onmessage").as_::<jsbind::Any>()
    }

    pub fn set_onmessage(&mut self, value: jsbind::Any) {
        self.inner.set("onmessage", value);
    }
}
impl RTCDataChannel {
    pub fn binary_type(&self) -> BinaryType {
        self.inner.get("binaryType").as_::<BinaryType>()
    }

    pub fn set_binary_type(&mut self, value: BinaryType) {
        self.inner.set("binaryType", value);
    }
}
impl RTCDataChannel {
    pub fn send(&self, data: jsbind::Any) -> jsbind::Undefined {
        self.inner
            .call("send", &[data.into()])
            .as_::<jsbind::Undefined>()
    }
}
impl RTCDataChannel {
    pub fn priority(&self) -> RTCPriorityType {
        self.inner.get("priority").as_::<RTCPriorityType>()
    }
}

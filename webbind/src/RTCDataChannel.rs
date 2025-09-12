use super::*;

/// The RTCDataChannel class.
/// [`RTCDataChannel`](https://developer.mozilla.org/en-US/docs/Web/API/RTCDataChannel)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct RTCDataChannel {
    inner: EventTarget,
}

impl FromVal for RTCDataChannel {
    fn from_val(v: &Any) -> Self {
        RTCDataChannel {
            inner: EventTarget::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
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

impl AsRef<Any> for RTCDataChannel {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for RTCDataChannel {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<RTCDataChannel> for Any {
    fn from(s: RTCDataChannel) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&RTCDataChannel> for Any {
    fn from(s: &RTCDataChannel) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(RTCDataChannel);

impl RTCDataChannel {
    /// Getter of the `label` attribute.
    /// [`RTCDataChannel.label`](https://developer.mozilla.org/en-US/docs/Web/API/RTCDataChannel/label)
    pub fn label(&self) -> JsString {
        self.inner.get("label").as_::<JsString>()
    }
}
impl RTCDataChannel {
    /// Getter of the `ordered` attribute.
    /// [`RTCDataChannel.ordered`](https://developer.mozilla.org/en-US/docs/Web/API/RTCDataChannel/ordered)
    pub fn ordered(&self) -> bool {
        self.inner.get("ordered").as_::<bool>()
    }
}
impl RTCDataChannel {
    /// Getter of the `maxPacketLifeTime` attribute.
    /// [`RTCDataChannel.maxPacketLifeTime`](https://developer.mozilla.org/en-US/docs/Web/API/RTCDataChannel/maxPacketLifeTime)
    pub fn max_packet_life_time(&self) -> u16 {
        self.inner.get("maxPacketLifeTime").as_::<u16>()
    }
}
impl RTCDataChannel {
    /// Getter of the `maxRetransmits` attribute.
    /// [`RTCDataChannel.maxRetransmits`](https://developer.mozilla.org/en-US/docs/Web/API/RTCDataChannel/maxRetransmits)
    pub fn max_retransmits(&self) -> u16 {
        self.inner.get("maxRetransmits").as_::<u16>()
    }
}
impl RTCDataChannel {
    /// Getter of the `protocol` attribute.
    /// [`RTCDataChannel.protocol`](https://developer.mozilla.org/en-US/docs/Web/API/RTCDataChannel/protocol)
    pub fn protocol(&self) -> JsString {
        self.inner.get("protocol").as_::<JsString>()
    }
}
impl RTCDataChannel {
    /// Getter of the `negotiated` attribute.
    /// [`RTCDataChannel.negotiated`](https://developer.mozilla.org/en-US/docs/Web/API/RTCDataChannel/negotiated)
    pub fn negotiated(&self) -> bool {
        self.inner.get("negotiated").as_::<bool>()
    }
}
impl RTCDataChannel {
    /// Getter of the `id` attribute.
    /// [`RTCDataChannel.id`](https://developer.mozilla.org/en-US/docs/Web/API/RTCDataChannel/id)
    pub fn id(&self) -> u16 {
        self.inner.get("id").as_::<u16>()
    }
}
impl RTCDataChannel {
    /// Getter of the `readyState` attribute.
    /// [`RTCDataChannel.readyState`](https://developer.mozilla.org/en-US/docs/Web/API/RTCDataChannel/readyState)
    pub fn ready_state(&self) -> RTCDataChannelState {
        self.inner.get("readyState").as_::<RTCDataChannelState>()
    }
}
impl RTCDataChannel {
    /// Getter of the `bufferedAmount` attribute.
    /// [`RTCDataChannel.bufferedAmount`](https://developer.mozilla.org/en-US/docs/Web/API/RTCDataChannel/bufferedAmount)
    pub fn buffered_amount(&self) -> u32 {
        self.inner.get("bufferedAmount").as_::<u32>()
    }
}
impl RTCDataChannel {
    /// Getter of the `bufferedAmountLowThreshold` attribute.
    /// [`RTCDataChannel.bufferedAmountLowThreshold`](https://developer.mozilla.org/en-US/docs/Web/API/RTCDataChannel/bufferedAmountLowThreshold)
    pub fn buffered_amount_low_threshold(&self) -> u32 {
        self.inner.get("bufferedAmountLowThreshold").as_::<u32>()
    }

    /// Setter of the `bufferedAmountLowThreshold` attribute.
    /// [`RTCDataChannel.bufferedAmountLowThreshold`](https://developer.mozilla.org/en-US/docs/Web/API/RTCDataChannel/bufferedAmountLowThreshold)
    pub fn set_buffered_amount_low_threshold(&mut self, value: u32) {
        self.inner.set("bufferedAmountLowThreshold", value);
    }
}
impl RTCDataChannel {
    /// Getter of the `onopen` attribute.
    /// [`RTCDataChannel.onopen`](https://developer.mozilla.org/en-US/docs/Web/API/RTCDataChannel/onopen)
    pub fn onopen(&self) -> Any {
        self.inner.get("onopen").as_::<Any>()
    }

    /// Setter of the `onopen` attribute.
    /// [`RTCDataChannel.onopen`](https://developer.mozilla.org/en-US/docs/Web/API/RTCDataChannel/onopen)
    pub fn set_onopen(&mut self, value: &Any) {
        self.inner.set("onopen", value);
    }
}
impl RTCDataChannel {
    /// Getter of the `onbufferedamountlow` attribute.
    /// [`RTCDataChannel.onbufferedamountlow`](https://developer.mozilla.org/en-US/docs/Web/API/RTCDataChannel/onbufferedamountlow)
    pub fn onbufferedamountlow(&self) -> Any {
        self.inner.get("onbufferedamountlow").as_::<Any>()
    }

    /// Setter of the `onbufferedamountlow` attribute.
    /// [`RTCDataChannel.onbufferedamountlow`](https://developer.mozilla.org/en-US/docs/Web/API/RTCDataChannel/onbufferedamountlow)
    pub fn set_onbufferedamountlow(&mut self, value: &Any) {
        self.inner.set("onbufferedamountlow", value);
    }
}
impl RTCDataChannel {
    /// Getter of the `onerror` attribute.
    /// [`RTCDataChannel.onerror`](https://developer.mozilla.org/en-US/docs/Web/API/RTCDataChannel/onerror)
    pub fn onerror(&self) -> Any {
        self.inner.get("onerror").as_::<Any>()
    }

    /// Setter of the `onerror` attribute.
    /// [`RTCDataChannel.onerror`](https://developer.mozilla.org/en-US/docs/Web/API/RTCDataChannel/onerror)
    pub fn set_onerror(&mut self, value: &Any) {
        self.inner.set("onerror", value);
    }
}
impl RTCDataChannel {
    /// Getter of the `onclosing` attribute.
    /// [`RTCDataChannel.onclosing`](https://developer.mozilla.org/en-US/docs/Web/API/RTCDataChannel/onclosing)
    pub fn onclosing(&self) -> Any {
        self.inner.get("onclosing").as_::<Any>()
    }

    /// Setter of the `onclosing` attribute.
    /// [`RTCDataChannel.onclosing`](https://developer.mozilla.org/en-US/docs/Web/API/RTCDataChannel/onclosing)
    pub fn set_onclosing(&mut self, value: &Any) {
        self.inner.set("onclosing", value);
    }
}
impl RTCDataChannel {
    /// Getter of the `onclose` attribute.
    /// [`RTCDataChannel.onclose`](https://developer.mozilla.org/en-US/docs/Web/API/RTCDataChannel/onclose)
    pub fn onclose(&self) -> Any {
        self.inner.get("onclose").as_::<Any>()
    }

    /// Setter of the `onclose` attribute.
    /// [`RTCDataChannel.onclose`](https://developer.mozilla.org/en-US/docs/Web/API/RTCDataChannel/onclose)
    pub fn set_onclose(&mut self, value: &Any) {
        self.inner.set("onclose", value);
    }
}
impl RTCDataChannel {
    /// Getter of the `onmessage` attribute.
    /// [`RTCDataChannel.onmessage`](https://developer.mozilla.org/en-US/docs/Web/API/RTCDataChannel/onmessage)
    pub fn onmessage(&self) -> Any {
        self.inner.get("onmessage").as_::<Any>()
    }

    /// Setter of the `onmessage` attribute.
    /// [`RTCDataChannel.onmessage`](https://developer.mozilla.org/en-US/docs/Web/API/RTCDataChannel/onmessage)
    pub fn set_onmessage(&mut self, value: &Any) {
        self.inner.set("onmessage", value);
    }
}
impl RTCDataChannel {
    /// Getter of the `binaryType` attribute.
    /// [`RTCDataChannel.binaryType`](https://developer.mozilla.org/en-US/docs/Web/API/RTCDataChannel/binaryType)
    pub fn binary_type(&self) -> BinaryType {
        self.inner.get("binaryType").as_::<BinaryType>()
    }

    /// Setter of the `binaryType` attribute.
    /// [`RTCDataChannel.binaryType`](https://developer.mozilla.org/en-US/docs/Web/API/RTCDataChannel/binaryType)
    pub fn set_binary_type(&mut self, value: &BinaryType) {
        self.inner.set("binaryType", value);
    }
}
impl RTCDataChannel {
    /// Getter of the `priority` attribute.
    /// [`RTCDataChannel.priority`](https://developer.mozilla.org/en-US/docs/Web/API/RTCDataChannel/priority)
    pub fn priority(&self) -> RTCPriorityType {
        self.inner.get("priority").as_::<RTCPriorityType>()
    }
}
impl RTCDataChannel {
    /// The close method.
    /// [`RTCDataChannel.close`](https://developer.mozilla.org/en-US/docs/Web/API/RTCDataChannel/close)
    pub fn close(&self) -> Undefined {
        self.inner.call("close", &[]).as_::<Undefined>()
    }
}
impl RTCDataChannel {
    /// The send method.
    /// [`RTCDataChannel.send`](https://developer.mozilla.org/en-US/docs/Web/API/RTCDataChannel/send)
    pub fn send(&self, data: &JsString) -> Undefined {
        self.inner.call("send", &[data.into()]).as_::<Undefined>()
    }
}
impl RTCDataChannel {
    /// The send method.
    /// [`RTCDataChannel.send`](https://developer.mozilla.org/en-US/docs/Web/API/RTCDataChannel/send)
    pub fn send_with_data(&self, data: &Blob) -> Undefined {
        self.inner.call("send", &[data.into()]).as_::<Undefined>()
    }
}
impl RTCDataChannel {
    /// The send method.
    /// [`RTCDataChannel.send`](https://developer.mozilla.org/en-US/docs/Web/API/RTCDataChannel/send)
    pub fn send_with_data_2(&self, data: &ArrayBuffer) -> Undefined {
        self.inner.call("send", &[data.into()]).as_::<Undefined>()
    }
}
impl RTCDataChannel {
    /// The send method.
    /// [`RTCDataChannel.send`](https://developer.mozilla.org/en-US/docs/Web/API/RTCDataChannel/send)
    pub fn send_with_data_3(&self, data: &Any) -> Undefined {
        self.inner.call("send", &[data.into()]).as_::<Undefined>()
    }
}

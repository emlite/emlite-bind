use super::*;




/// The RTCDataChannelInit dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct RTCDataChannelInit {
    inner: Any,
}

impl FromVal for RTCDataChannelInit {
    fn from_val(v: &Any) -> Self {
        RTCDataChannelInit { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for RTCDataChannelInit {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for RTCDataChannelInit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for RTCDataChannelInit {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for RTCDataChannelInit {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<RTCDataChannelInit> for Any {
    fn from(s: RTCDataChannelInit) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&RTCDataChannelInit> for Any {
    fn from(s: &RTCDataChannelInit) -> Any {
        s.inner.clone()
    }
}

impl RTCDataChannelInit {
    /// Getter of the `ordered` attribute.
    pub fn ordered(&self) -> bool {
        self.inner.get("ordered").as_::<bool>()
    }

    /// Setter of the `ordered` attribute.
    pub fn set_ordered(&mut self, value: bool) {
        self.inner.set("ordered", value);
    }
}
impl RTCDataChannelInit {
    /// Getter of the `maxPacketLifeTime` attribute.
    pub fn max_packet_life_time(&self) -> u16 {
        self.inner.get("maxPacketLifeTime").as_::<u16>()
    }

    /// Setter of the `maxPacketLifeTime` attribute.
    pub fn set_max_packet_life_time(&mut self, value: u16) {
        self.inner.set("maxPacketLifeTime", value);
    }
}
impl RTCDataChannelInit {
    /// Getter of the `maxRetransmits` attribute.
    pub fn max_retransmits(&self) -> u16 {
        self.inner.get("maxRetransmits").as_::<u16>()
    }

    /// Setter of the `maxRetransmits` attribute.
    pub fn set_max_retransmits(&mut self, value: u16) {
        self.inner.set("maxRetransmits", value);
    }
}
impl RTCDataChannelInit {
    /// Getter of the `protocol` attribute.
    pub fn protocol(&self) -> JsString {
        self.inner.get("protocol").as_::<JsString>()
    }

    /// Setter of the `protocol` attribute.
    pub fn set_protocol(&mut self, value: &JsString) {
        self.inner.set("protocol", value);
    }
}
impl RTCDataChannelInit {
    /// Getter of the `negotiated` attribute.
    pub fn negotiated(&self) -> bool {
        self.inner.get("negotiated").as_::<bool>()
    }

    /// Setter of the `negotiated` attribute.
    pub fn set_negotiated(&mut self, value: bool) {
        self.inner.set("negotiated", value);
    }
}
impl RTCDataChannelInit {
    /// Getter of the `id` attribute.
    pub fn id(&self) -> u16 {
        self.inner.get("id").as_::<u16>()
    }

    /// Setter of the `id` attribute.
    pub fn set_id(&mut self, value: u16) {
        self.inner.set("id", value);
    }
}

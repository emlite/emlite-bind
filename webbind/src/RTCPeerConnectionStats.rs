use super::*;

/// The RTCPeerConnectionStats dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct RTCPeerConnectionStats {
    inner: Any,
}

impl FromVal for RTCPeerConnectionStats {
    fn from_val(v: &Any) -> Self {
        RTCPeerConnectionStats { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for RTCPeerConnectionStats {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for RTCPeerConnectionStats {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for RTCPeerConnectionStats {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for RTCPeerConnectionStats {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<RTCPeerConnectionStats> for Any {
    fn from(s: RTCPeerConnectionStats) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&RTCPeerConnectionStats> for Any {
    fn from(s: &RTCPeerConnectionStats) -> Any {
        s.inner.clone()
    }
}

impl RTCPeerConnectionStats {
    /// Getter of the `dataChannelsOpened` attribute.
    pub fn data_channels_opened(&self) -> u32 {
        self.inner.get("dataChannelsOpened").as_::<u32>()
    }

    /// Setter of the `dataChannelsOpened` attribute.
    pub fn set_data_channels_opened(&mut self, value: u32) {
        self.inner.set("dataChannelsOpened", value);
    }
}
impl RTCPeerConnectionStats {
    /// Getter of the `dataChannelsClosed` attribute.
    pub fn data_channels_closed(&self) -> u32 {
        self.inner.get("dataChannelsClosed").as_::<u32>()
    }

    /// Setter of the `dataChannelsClosed` attribute.
    pub fn set_data_channels_closed(&mut self, value: u32) {
        self.inner.set("dataChannelsClosed", value);
    }
}

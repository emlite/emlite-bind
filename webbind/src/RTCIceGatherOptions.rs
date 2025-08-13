use super::*;




/// The RTCIceGatherOptions dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct RTCIceGatherOptions {
    inner: Any,
}

impl FromVal for RTCIceGatherOptions {
    fn from_val(v: &Any) -> Self {
        RTCIceGatherOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for RTCIceGatherOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for RTCIceGatherOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for RTCIceGatherOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for RTCIceGatherOptions {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<RTCIceGatherOptions> for Any {
    fn from(s: RTCIceGatherOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&RTCIceGatherOptions> for Any {
    fn from(s: &RTCIceGatherOptions) -> Any {
        s.inner.clone()
    }
}

impl RTCIceGatherOptions {
    /// Getter of the `gatherPolicy` attribute.
    pub fn gather_policy(&self) -> RTCIceTransportPolicy {
        self.inner.get("gatherPolicy").as_::<RTCIceTransportPolicy>()
    }

    /// Setter of the `gatherPolicy` attribute.
    pub fn set_gather_policy(&mut self, value: &RTCIceTransportPolicy) {
        self.inner.set("gatherPolicy", value);
    }
}
impl RTCIceGatherOptions {
    /// Getter of the `iceServers` attribute.
    pub fn ice_servers(&self) -> TypedArray<RTCIceServer> {
        self.inner.get("iceServers").as_::<TypedArray<RTCIceServer>>()
    }

    /// Setter of the `iceServers` attribute.
    pub fn set_ice_servers(&mut self, value: &TypedArray<RTCIceServer>) {
        self.inner.set("iceServers", value);
    }
}

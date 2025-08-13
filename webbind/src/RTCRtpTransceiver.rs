use super::*;




/// The RTCRtpTransceiver class.
/// [`RTCRtpTransceiver`](https://developer.mozilla.org/en-US/docs/Web/API/RTCRtpTransceiver)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct RTCRtpTransceiver {
    inner: Any,
}

impl FromVal for RTCRtpTransceiver {
    fn from_val(v: &Any) -> Self {
        RTCRtpTransceiver { inner: Any::from_val(v) }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for RTCRtpTransceiver {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for RTCRtpTransceiver {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for RTCRtpTransceiver {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for RTCRtpTransceiver {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<RTCRtpTransceiver> for Any {
    fn from(s: RTCRtpTransceiver) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&RTCRtpTransceiver> for Any {
    fn from(s: &RTCRtpTransceiver) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(RTCRtpTransceiver);


impl RTCRtpTransceiver {
    /// Getter of the `mid` attribute.
    /// [`RTCRtpTransceiver.mid`](https://developer.mozilla.org/en-US/docs/Web/API/RTCRtpTransceiver/mid)
    pub fn mid(&self) -> JsString {
        self.inner.get("mid").as_::<JsString>()
    }

}
impl RTCRtpTransceiver {
    /// Getter of the `sender` attribute.
    /// [`RTCRtpTransceiver.sender`](https://developer.mozilla.org/en-US/docs/Web/API/RTCRtpTransceiver/sender)
    pub fn sender(&self) -> RTCRtpSender {
        self.inner.get("sender").as_::<RTCRtpSender>()
    }

}
impl RTCRtpTransceiver {
    /// Getter of the `receiver` attribute.
    /// [`RTCRtpTransceiver.receiver`](https://developer.mozilla.org/en-US/docs/Web/API/RTCRtpTransceiver/receiver)
    pub fn receiver(&self) -> RTCRtpReceiver {
        self.inner.get("receiver").as_::<RTCRtpReceiver>()
    }

}
impl RTCRtpTransceiver {
    /// Getter of the `direction` attribute.
    /// [`RTCRtpTransceiver.direction`](https://developer.mozilla.org/en-US/docs/Web/API/RTCRtpTransceiver/direction)
    pub fn direction(&self) -> RTCRtpTransceiverDirection {
        self.inner.get("direction").as_::<RTCRtpTransceiverDirection>()
    }

    /// Setter of the `direction` attribute.
    /// [`RTCRtpTransceiver.direction`](https://developer.mozilla.org/en-US/docs/Web/API/RTCRtpTransceiver/direction)
    pub fn set_direction(&mut self, value: &RTCRtpTransceiverDirection) {
        self.inner.set("direction", value);
    }
}
impl RTCRtpTransceiver {
    /// Getter of the `currentDirection` attribute.
    /// [`RTCRtpTransceiver.currentDirection`](https://developer.mozilla.org/en-US/docs/Web/API/RTCRtpTransceiver/currentDirection)
    pub fn current_direction(&self) -> RTCRtpTransceiverDirection {
        self.inner.get("currentDirection").as_::<RTCRtpTransceiverDirection>()
    }

}
impl RTCRtpTransceiver {
    /// The stop method.
    /// [`RTCRtpTransceiver.stop`](https://developer.mozilla.org/en-US/docs/Web/API/RTCRtpTransceiver/stop)
    pub fn stop(&self, ) -> Undefined {
        self.inner.call("stop", &[]).as_::<Undefined>()
    }
}
impl RTCRtpTransceiver {
    /// The setCodecPreferences method.
    /// [`RTCRtpTransceiver.setCodecPreferences`](https://developer.mozilla.org/en-US/docs/Web/API/RTCRtpTransceiver/setCodecPreferences)
    pub fn set_codec_preferences(&self, codecs: &TypedArray<RTCRtpCodec>) -> Undefined {
        self.inner.call("setCodecPreferences", &[codecs.into(), ]).as_::<Undefined>()
    }
}

use super::*;




/// The AudioEncoder class.
/// [`AudioEncoder`](https://developer.mozilla.org/en-US/docs/Web/API/AudioEncoder)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct AudioEncoder {
    inner: EventTarget,
}

impl FromVal for AudioEncoder {
    fn from_val(v: &Any) -> Self {
        AudioEncoder { inner: EventTarget::from_val(v) }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for AudioEncoder {
    type Target = EventTarget;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for AudioEncoder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for AudioEncoder {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for AudioEncoder {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<AudioEncoder> for Any {
    fn from(s: AudioEncoder) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&AudioEncoder> for Any {
    fn from(s: &AudioEncoder) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(AudioEncoder);



impl AudioEncoder {
    /// The `new AudioEncoder(..)` constructor, creating a new AudioEncoder instance
    pub fn new(init: &AudioEncoderInit) -> AudioEncoder {
        Self {
            inner: Any::global("AudioEncoder").new(&[init.into()]).as_::<EventTarget>(),
        }
    }

}
impl AudioEncoder {
    /// Getter of the `state` attribute.
    /// [`AudioEncoder.state`](https://developer.mozilla.org/en-US/docs/Web/API/AudioEncoder/state)
    pub fn state(&self) -> CodecState {
        self.inner.get("state").as_::<CodecState>()
    }

}
impl AudioEncoder {
    /// Getter of the `encodeQueueSize` attribute.
    /// [`AudioEncoder.encodeQueueSize`](https://developer.mozilla.org/en-US/docs/Web/API/AudioEncoder/encodeQueueSize)
    pub fn encode_queue_size(&self) -> u32 {
        self.inner.get("encodeQueueSize").as_::<u32>()
    }

}
impl AudioEncoder {
    /// Getter of the `ondequeue` attribute.
    /// [`AudioEncoder.ondequeue`](https://developer.mozilla.org/en-US/docs/Web/API/AudioEncoder/ondequeue)
    pub fn ondequeue(&self) -> Any {
        self.inner.get("ondequeue").as_::<Any>()
    }

    /// Setter of the `ondequeue` attribute.
    /// [`AudioEncoder.ondequeue`](https://developer.mozilla.org/en-US/docs/Web/API/AudioEncoder/ondequeue)
    pub fn set_ondequeue(&mut self, value: &Any) {
        self.inner.set("ondequeue", value);
    }
}
impl AudioEncoder {
    /// The configure method.
    /// [`AudioEncoder.configure`](https://developer.mozilla.org/en-US/docs/Web/API/AudioEncoder/configure)
    pub fn configure(&self, config: &AudioEncoderConfig) -> Undefined {
        self.inner.call("configure", &[config.into(), ]).as_::<Undefined>()
    }
}
impl AudioEncoder {
    /// The encode method.
    /// [`AudioEncoder.encode`](https://developer.mozilla.org/en-US/docs/Web/API/AudioEncoder/encode)
    pub fn encode(&self, data: &AudioData) -> Undefined {
        self.inner.call("encode", &[data.into(), ]).as_::<Undefined>()
    }
}
impl AudioEncoder {
    /// The flush method.
    /// [`AudioEncoder.flush`](https://developer.mozilla.org/en-US/docs/Web/API/AudioEncoder/flush)
    pub fn flush(&self, ) -> Promise<Undefined> {
        self.inner.call("flush", &[]).as_::<Promise<Undefined>>()
    }
}
impl AudioEncoder {
    /// The reset method.
    /// [`AudioEncoder.reset`](https://developer.mozilla.org/en-US/docs/Web/API/AudioEncoder/reset)
    pub fn reset(&self, ) -> Undefined {
        self.inner.call("reset", &[]).as_::<Undefined>()
    }
}
impl AudioEncoder {
    /// The close method.
    /// [`AudioEncoder.close`](https://developer.mozilla.org/en-US/docs/Web/API/AudioEncoder/close)
    pub fn close(&self, ) -> Undefined {
        self.inner.call("close", &[]).as_::<Undefined>()
    }
}
impl AudioEncoder {
    /// The isConfigSupported method.
    /// [`AudioEncoder.isConfigSupported`](https://developer.mozilla.org/en-US/docs/Web/API/AudioEncoder/isConfigSupported)
    pub fn is_config_supported(config: &AudioEncoderConfig) -> Promise<AudioEncoderSupport> {
        Any::global("AudioEncoder").call("isConfigSupported", &[config.into(), ]).as_::<Promise<AudioEncoderSupport>>()
    }
}

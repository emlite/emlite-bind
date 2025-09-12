use super::*;

/// The AudioDecoder class.
/// [`AudioDecoder`](https://developer.mozilla.org/en-US/docs/Web/API/AudioDecoder)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct AudioDecoder {
    inner: EventTarget,
}

impl FromVal for AudioDecoder {
    fn from_val(v: &Any) -> Self {
        AudioDecoder {
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

impl core::ops::Deref for AudioDecoder {
    type Target = EventTarget;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for AudioDecoder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for AudioDecoder {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for AudioDecoder {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<AudioDecoder> for Any {
    fn from(s: AudioDecoder) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&AudioDecoder> for Any {
    fn from(s: &AudioDecoder) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(AudioDecoder);

impl AudioDecoder {
    /// Getter of the `state` attribute.
    /// [`AudioDecoder.state`](https://developer.mozilla.org/en-US/docs/Web/API/AudioDecoder/state)
    pub fn state(&self) -> CodecState {
        self.inner.get("state").as_::<CodecState>()
    }
}
impl AudioDecoder {
    /// Getter of the `decodeQueueSize` attribute.
    /// [`AudioDecoder.decodeQueueSize`](https://developer.mozilla.org/en-US/docs/Web/API/AudioDecoder/decodeQueueSize)
    pub fn decode_queue_size(&self) -> u32 {
        self.inner.get("decodeQueueSize").as_::<u32>()
    }
}
impl AudioDecoder {
    /// Getter of the `ondequeue` attribute.
    /// [`AudioDecoder.ondequeue`](https://developer.mozilla.org/en-US/docs/Web/API/AudioDecoder/ondequeue)
    pub fn ondequeue(&self) -> Any {
        self.inner.get("ondequeue").as_::<Any>()
    }

    /// Setter of the `ondequeue` attribute.
    /// [`AudioDecoder.ondequeue`](https://developer.mozilla.org/en-US/docs/Web/API/AudioDecoder/ondequeue)
    pub fn set_ondequeue(&mut self, value: &Any) {
        self.inner.set("ondequeue", value);
    }
}

impl AudioDecoder {
    /// The `new AudioDecoder(..)` constructor, creating a new AudioDecoder instance
    pub fn new(init: &AudioDecoderInit) -> AudioDecoder {
        Self {
            inner: Any::global("AudioDecoder")
                .new(&[init.into()])
                .as_::<EventTarget>(),
        }
    }
}
impl AudioDecoder {
    /// The configure method.
    /// [`AudioDecoder.configure`](https://developer.mozilla.org/en-US/docs/Web/API/AudioDecoder/configure)
    pub fn configure(&self, config: &AudioDecoderConfig) -> Undefined {
        self.inner
            .call("configure", &[config.into()])
            .as_::<Undefined>()
    }
}
impl AudioDecoder {
    /// The decode method.
    /// [`AudioDecoder.decode`](https://developer.mozilla.org/en-US/docs/Web/API/AudioDecoder/decode)
    pub fn decode(&self, chunk: &EncodedAudioChunk) -> Undefined {
        self.inner
            .call("decode", &[chunk.into()])
            .as_::<Undefined>()
    }
}
impl AudioDecoder {
    /// The flush method.
    /// [`AudioDecoder.flush`](https://developer.mozilla.org/en-US/docs/Web/API/AudioDecoder/flush)
    pub fn flush(&self) -> Promise<Undefined> {
        self.inner.call("flush", &[]).as_::<Promise<Undefined>>()
    }
}
impl AudioDecoder {
    /// The reset method.
    /// [`AudioDecoder.reset`](https://developer.mozilla.org/en-US/docs/Web/API/AudioDecoder/reset)
    pub fn reset(&self) -> Undefined {
        self.inner.call("reset", &[]).as_::<Undefined>()
    }
}
impl AudioDecoder {
    /// The close method.
    /// [`AudioDecoder.close`](https://developer.mozilla.org/en-US/docs/Web/API/AudioDecoder/close)
    pub fn close(&self) -> Undefined {
        self.inner.call("close", &[]).as_::<Undefined>()
    }
}
impl AudioDecoder {
    /// The isConfigSupported method.
    /// [`AudioDecoder.isConfigSupported`](https://developer.mozilla.org/en-US/docs/Web/API/AudioDecoder/isConfigSupported)
    pub fn is_config_supported(config: &AudioDecoderConfig) -> Promise<AudioDecoderSupport> {
        Any::global("AudioDecoder")
            .call("isConfigSupported", &[config.into()])
            .as_::<Promise<AudioDecoderSupport>>()
    }
}

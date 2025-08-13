use super::*;




/// The VideoDecoder class.
/// [`VideoDecoder`](https://developer.mozilla.org/en-US/docs/Web/API/VideoDecoder)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct VideoDecoder {
    inner: EventTarget,
}

impl FromVal for VideoDecoder {
    fn from_val(v: &Any) -> Self {
        VideoDecoder { inner: EventTarget::from_val(v) }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for VideoDecoder {
    type Target = EventTarget;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for VideoDecoder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for VideoDecoder {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for VideoDecoder {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<VideoDecoder> for Any {
    fn from(s: VideoDecoder) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&VideoDecoder> for Any {
    fn from(s: &VideoDecoder) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(VideoDecoder);



impl VideoDecoder {
    /// The `new VideoDecoder(..)` constructor, creating a new VideoDecoder instance
    pub fn new(init: &VideoDecoderInit) -> VideoDecoder {
        Self {
            inner: Any::global("VideoDecoder").new(&[init.into()]).as_::<EventTarget>(),
        }
    }

}
impl VideoDecoder {
    /// Getter of the `state` attribute.
    /// [`VideoDecoder.state`](https://developer.mozilla.org/en-US/docs/Web/API/VideoDecoder/state)
    pub fn state(&self) -> CodecState {
        self.inner.get("state").as_::<CodecState>()
    }

}
impl VideoDecoder {
    /// Getter of the `decodeQueueSize` attribute.
    /// [`VideoDecoder.decodeQueueSize`](https://developer.mozilla.org/en-US/docs/Web/API/VideoDecoder/decodeQueueSize)
    pub fn decode_queue_size(&self) -> u32 {
        self.inner.get("decodeQueueSize").as_::<u32>()
    }

}
impl VideoDecoder {
    /// Getter of the `ondequeue` attribute.
    /// [`VideoDecoder.ondequeue`](https://developer.mozilla.org/en-US/docs/Web/API/VideoDecoder/ondequeue)
    pub fn ondequeue(&self) -> Any {
        self.inner.get("ondequeue").as_::<Any>()
    }

    /// Setter of the `ondequeue` attribute.
    /// [`VideoDecoder.ondequeue`](https://developer.mozilla.org/en-US/docs/Web/API/VideoDecoder/ondequeue)
    pub fn set_ondequeue(&mut self, value: &Any) {
        self.inner.set("ondequeue", value);
    }
}
impl VideoDecoder {
    /// The configure method.
    /// [`VideoDecoder.configure`](https://developer.mozilla.org/en-US/docs/Web/API/VideoDecoder/configure)
    pub fn configure(&self, config: &VideoDecoderConfig) -> Undefined {
        self.inner.call("configure", &[config.into(), ]).as_::<Undefined>()
    }
}
impl VideoDecoder {
    /// The decode method.
    /// [`VideoDecoder.decode`](https://developer.mozilla.org/en-US/docs/Web/API/VideoDecoder/decode)
    pub fn decode(&self, chunk: &EncodedVideoChunk) -> Undefined {
        self.inner.call("decode", &[chunk.into(), ]).as_::<Undefined>()
    }
}
impl VideoDecoder {
    /// The flush method.
    /// [`VideoDecoder.flush`](https://developer.mozilla.org/en-US/docs/Web/API/VideoDecoder/flush)
    pub fn flush(&self, ) -> Promise<Undefined> {
        self.inner.call("flush", &[]).as_::<Promise<Undefined>>()
    }
}
impl VideoDecoder {
    /// The reset method.
    /// [`VideoDecoder.reset`](https://developer.mozilla.org/en-US/docs/Web/API/VideoDecoder/reset)
    pub fn reset(&self, ) -> Undefined {
        self.inner.call("reset", &[]).as_::<Undefined>()
    }
}
impl VideoDecoder {
    /// The close method.
    /// [`VideoDecoder.close`](https://developer.mozilla.org/en-US/docs/Web/API/VideoDecoder/close)
    pub fn close(&self, ) -> Undefined {
        self.inner.call("close", &[]).as_::<Undefined>()
    }
}
impl VideoDecoder {
    /// The isConfigSupported method.
    /// [`VideoDecoder.isConfigSupported`](https://developer.mozilla.org/en-US/docs/Web/API/VideoDecoder/isConfigSupported)
    pub fn is_config_supported(config: &VideoDecoderConfig) -> Promise<VideoDecoderSupport> {
        Any::global("VideoDecoder").call("isConfigSupported", &[config.into(), ]).as_::<Promise<VideoDecoderSupport>>()
    }
}

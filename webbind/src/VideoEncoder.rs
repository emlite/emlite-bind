use super::*;




/// The VideoEncoder class.
/// [`VideoEncoder`](https://developer.mozilla.org/en-US/docs/Web/API/VideoEncoder)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct VideoEncoder {
    inner: EventTarget,
}

impl FromVal for VideoEncoder {
    fn from_val(v: &Any) -> Self {
        VideoEncoder { inner: EventTarget::from_val(v) }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for VideoEncoder {
    type Target = EventTarget;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for VideoEncoder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for VideoEncoder {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for VideoEncoder {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<VideoEncoder> for Any {
    fn from(s: VideoEncoder) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&VideoEncoder> for Any {
    fn from(s: &VideoEncoder) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(VideoEncoder);



impl VideoEncoder {
    /// The `new VideoEncoder(..)` constructor, creating a new VideoEncoder instance
    pub fn new(init: &VideoEncoderInit) -> VideoEncoder {
        Self {
            inner: Any::global("VideoEncoder").new(&[init.into()]).as_::<EventTarget>(),
        }
    }

}
impl VideoEncoder {
    /// Getter of the `state` attribute.
    /// [`VideoEncoder.state`](https://developer.mozilla.org/en-US/docs/Web/API/VideoEncoder/state)
    pub fn state(&self) -> CodecState {
        self.inner.get("state").as_::<CodecState>()
    }

}
impl VideoEncoder {
    /// Getter of the `encodeQueueSize` attribute.
    /// [`VideoEncoder.encodeQueueSize`](https://developer.mozilla.org/en-US/docs/Web/API/VideoEncoder/encodeQueueSize)
    pub fn encode_queue_size(&self) -> u32 {
        self.inner.get("encodeQueueSize").as_::<u32>()
    }

}
impl VideoEncoder {
    /// Getter of the `ondequeue` attribute.
    /// [`VideoEncoder.ondequeue`](https://developer.mozilla.org/en-US/docs/Web/API/VideoEncoder/ondequeue)
    pub fn ondequeue(&self) -> Any {
        self.inner.get("ondequeue").as_::<Any>()
    }

    /// Setter of the `ondequeue` attribute.
    /// [`VideoEncoder.ondequeue`](https://developer.mozilla.org/en-US/docs/Web/API/VideoEncoder/ondequeue)
    pub fn set_ondequeue(&mut self, value: &Any) {
        self.inner.set("ondequeue", value);
    }
}
impl VideoEncoder {
    /// The configure method.
    /// [`VideoEncoder.configure`](https://developer.mozilla.org/en-US/docs/Web/API/VideoEncoder/configure)
    pub fn configure(&self, config: &VideoEncoderConfig) -> Undefined {
        self.inner.call("configure", &[config.into(), ]).as_::<Undefined>()
    }
}
impl VideoEncoder {
    /// The encode method.
    /// [`VideoEncoder.encode`](https://developer.mozilla.org/en-US/docs/Web/API/VideoEncoder/encode)
    pub fn encode0(&self, frame: &VideoFrame) -> Undefined {
        self.inner.call("encode", &[frame.into(), ]).as_::<Undefined>()
    }
    /// The encode method.
    /// [`VideoEncoder.encode`](https://developer.mozilla.org/en-US/docs/Web/API/VideoEncoder/encode)
    pub fn encode1(&self, frame: &VideoFrame, options: &VideoEncoderEncodeOptions) -> Undefined {
        self.inner.call("encode", &[frame.into(), options.into(), ]).as_::<Undefined>()
    }
}
impl VideoEncoder {
    /// The flush method.
    /// [`VideoEncoder.flush`](https://developer.mozilla.org/en-US/docs/Web/API/VideoEncoder/flush)
    pub fn flush(&self, ) -> Promise<Undefined> {
        self.inner.call("flush", &[]).as_::<Promise<Undefined>>()
    }
}
impl VideoEncoder {
    /// The reset method.
    /// [`VideoEncoder.reset`](https://developer.mozilla.org/en-US/docs/Web/API/VideoEncoder/reset)
    pub fn reset(&self, ) -> Undefined {
        self.inner.call("reset", &[]).as_::<Undefined>()
    }
}
impl VideoEncoder {
    /// The close method.
    /// [`VideoEncoder.close`](https://developer.mozilla.org/en-US/docs/Web/API/VideoEncoder/close)
    pub fn close(&self, ) -> Undefined {
        self.inner.call("close", &[]).as_::<Undefined>()
    }
}
impl VideoEncoder {
    /// The isConfigSupported method.
    /// [`VideoEncoder.isConfigSupported`](https://developer.mozilla.org/en-US/docs/Web/API/VideoEncoder/isConfigSupported)
    pub fn is_config_supported(config: &VideoEncoderConfig) -> Promise<VideoEncoderSupport> {
        Any::global("VideoEncoder").call("isConfigSupported", &[config.into(), ]).as_::<Promise<VideoEncoderSupport>>()
    }
}

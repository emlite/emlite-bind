use super::*;




/// The AudioBufferSourceNode class.
/// [`AudioBufferSourceNode`](https://developer.mozilla.org/en-US/docs/Web/API/AudioBufferSourceNode)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct AudioBufferSourceNode {
    inner: AudioScheduledSourceNode,
}

impl FromVal for AudioBufferSourceNode {
    fn from_val(v: &Any) -> Self {
        AudioBufferSourceNode { inner: AudioScheduledSourceNode::from_val(v) }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for AudioBufferSourceNode {
    type Target = AudioScheduledSourceNode;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for AudioBufferSourceNode {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for AudioBufferSourceNode {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for AudioBufferSourceNode {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<AudioBufferSourceNode> for Any {
    fn from(s: AudioBufferSourceNode) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&AudioBufferSourceNode> for Any {
    fn from(s: &AudioBufferSourceNode) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(AudioBufferSourceNode);



impl AudioBufferSourceNode {
    /// The `new AudioBufferSourceNode(..)` constructor, creating a new AudioBufferSourceNode instance
    pub fn new0(context: &BaseAudioContext) -> AudioBufferSourceNode {
        Self {
            inner: Any::global("AudioBufferSourceNode").new(&[context.into()]).as_::<AudioScheduledSourceNode>(),
        }
    }

    /// The `new AudioBufferSourceNode(..)` constructor, creating a new AudioBufferSourceNode instance
    pub fn new1(context: &BaseAudioContext, options: &AudioBufferSourceOptions) -> AudioBufferSourceNode {
        Self {
            inner: Any::global("AudioBufferSourceNode").new(&[context.into(), options.into()]).as_::<AudioScheduledSourceNode>(),
        }
    }

}
impl AudioBufferSourceNode {
    /// Getter of the `buffer` attribute.
    /// [`AudioBufferSourceNode.buffer`](https://developer.mozilla.org/en-US/docs/Web/API/AudioBufferSourceNode/buffer)
    pub fn buffer(&self) -> AudioBuffer {
        self.inner.get("buffer").as_::<AudioBuffer>()
    }

    /// Setter of the `buffer` attribute.
    /// [`AudioBufferSourceNode.buffer`](https://developer.mozilla.org/en-US/docs/Web/API/AudioBufferSourceNode/buffer)
    pub fn set_buffer(&mut self, value: &AudioBuffer) {
        self.inner.set("buffer", value);
    }
}
impl AudioBufferSourceNode {
    /// Getter of the `playbackRate` attribute.
    /// [`AudioBufferSourceNode.playbackRate`](https://developer.mozilla.org/en-US/docs/Web/API/AudioBufferSourceNode/playbackRate)
    pub fn playback_rate(&self) -> AudioParam {
        self.inner.get("playbackRate").as_::<AudioParam>()
    }

}
impl AudioBufferSourceNode {
    /// Getter of the `detune` attribute.
    /// [`AudioBufferSourceNode.detune`](https://developer.mozilla.org/en-US/docs/Web/API/AudioBufferSourceNode/detune)
    pub fn detune(&self) -> AudioParam {
        self.inner.get("detune").as_::<AudioParam>()
    }

}
impl AudioBufferSourceNode {
    /// Getter of the `loop` attribute.
    /// [`AudioBufferSourceNode.loop`](https://developer.mozilla.org/en-US/docs/Web/API/AudioBufferSourceNode/loop)
    pub fn loop_(&self) -> bool {
        self.inner.get("loop").as_::<bool>()
    }

    /// Setter of the `loop` attribute.
    /// [`AudioBufferSourceNode.loop`](https://developer.mozilla.org/en-US/docs/Web/API/AudioBufferSourceNode/loop)
    pub fn set_loop_(&mut self, value: bool) {
        self.inner.set("loop", value);
    }
}
impl AudioBufferSourceNode {
    /// Getter of the `loopStart` attribute.
    /// [`AudioBufferSourceNode.loopStart`](https://developer.mozilla.org/en-US/docs/Web/API/AudioBufferSourceNode/loopStart)
    pub fn loop_start(&self) -> f64 {
        self.inner.get("loopStart").as_::<f64>()
    }

    /// Setter of the `loopStart` attribute.
    /// [`AudioBufferSourceNode.loopStart`](https://developer.mozilla.org/en-US/docs/Web/API/AudioBufferSourceNode/loopStart)
    pub fn set_loop_start(&mut self, value: f64) {
        self.inner.set("loopStart", value);
    }
}
impl AudioBufferSourceNode {
    /// Getter of the `loopEnd` attribute.
    /// [`AudioBufferSourceNode.loopEnd`](https://developer.mozilla.org/en-US/docs/Web/API/AudioBufferSourceNode/loopEnd)
    pub fn loop_end(&self) -> f64 {
        self.inner.get("loopEnd").as_::<f64>()
    }

    /// Setter of the `loopEnd` attribute.
    /// [`AudioBufferSourceNode.loopEnd`](https://developer.mozilla.org/en-US/docs/Web/API/AudioBufferSourceNode/loopEnd)
    pub fn set_loop_end(&mut self, value: f64) {
        self.inner.set("loopEnd", value);
    }
}
impl AudioBufferSourceNode {
    /// The start method.
    /// [`AudioBufferSourceNode.start`](https://developer.mozilla.org/en-US/docs/Web/API/AudioBufferSourceNode/start)
    pub fn start0(&self, ) -> Undefined {
        self.inner.call("start", &[]).as_::<Undefined>()
    }
    /// The start method.
    /// [`AudioBufferSourceNode.start`](https://developer.mozilla.org/en-US/docs/Web/API/AudioBufferSourceNode/start)
    pub fn start1(&self, when: f64) -> Undefined {
        self.inner.call("start", &[when.into(), ]).as_::<Undefined>()
    }
    /// The start method.
    /// [`AudioBufferSourceNode.start`](https://developer.mozilla.org/en-US/docs/Web/API/AudioBufferSourceNode/start)
    pub fn start2(&self, when: f64, offset: f64) -> Undefined {
        self.inner.call("start", &[when.into(), offset.into(), ]).as_::<Undefined>()
    }
    /// The start method.
    /// [`AudioBufferSourceNode.start`](https://developer.mozilla.org/en-US/docs/Web/API/AudioBufferSourceNode/start)
    pub fn start3(&self, when: f64, offset: f64, duration: f64) -> Undefined {
        self.inner.call("start", &[when.into(), offset.into(), duration.into(), ]).as_::<Undefined>()
    }
}

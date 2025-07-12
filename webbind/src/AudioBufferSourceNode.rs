use super::*;

#[derive(Clone, Debug)]
pub struct AudioBufferSourceNode {
    inner: AudioScheduledSourceNode,
}
impl FromVal for AudioBufferSourceNode {
    fn from_val(v: &emlite::Val) -> Self {
        AudioBufferSourceNode {
            inner: AudioScheduledSourceNode::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for AudioBufferSourceNode {
    type Target = AudioScheduledSourceNode;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for AudioBufferSourceNode {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<AudioBufferSourceNode> for emlite::Val {
    fn from(s: AudioBufferSourceNode) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl AudioBufferSourceNode {
    pub fn new0(context: BaseAudioContext) -> AudioBufferSourceNode {
        Self {
            inner: emlite::Val::global("AudioBufferSourceNode")
                .new(&[context.into()])
                .as_::<AudioScheduledSourceNode>(),
        }
    }

    pub fn new1(context: BaseAudioContext, options: jsbind::Any) -> AudioBufferSourceNode {
        Self {
            inner: emlite::Val::global("AudioBufferSourceNode")
                .new(&[context.into(), options.into()])
                .as_::<AudioScheduledSourceNode>(),
        }
    }
}
impl AudioBufferSourceNode {
    pub fn buffer(&self) -> AudioBuffer {
        self.inner.get("buffer").as_::<AudioBuffer>()
    }

    pub fn set_buffer(&mut self, value: AudioBuffer) {
        self.inner.set("buffer", value);
    }
}
impl AudioBufferSourceNode {
    pub fn playback_rate(&self) -> AudioParam {
        self.inner.get("playbackRate").as_::<AudioParam>()
    }
}
impl AudioBufferSourceNode {
    pub fn detune(&self) -> AudioParam {
        self.inner.get("detune").as_::<AudioParam>()
    }
}
impl AudioBufferSourceNode {
    pub fn loop_(&self) -> bool {
        self.inner.get("loop").as_::<bool>()
    }

    pub fn set_loop_(&mut self, value: bool) {
        self.inner.set("loop", value);
    }
}
impl AudioBufferSourceNode {
    pub fn loop_start(&self) -> f64 {
        self.inner.get("loopStart").as_::<f64>()
    }

    pub fn set_loop_start(&mut self, value: f64) {
        self.inner.set("loopStart", value);
    }
}
impl AudioBufferSourceNode {
    pub fn loop_end(&self) -> f64 {
        self.inner.get("loopEnd").as_::<f64>()
    }

    pub fn set_loop_end(&mut self, value: f64) {
        self.inner.set("loopEnd", value);
    }
}
impl AudioBufferSourceNode {
    pub fn start0(&self) -> jsbind::Undefined {
        self.inner.call("start", &[]).as_::<jsbind::Undefined>()
    }

    pub fn start1(&self, when: f64) -> jsbind::Undefined {
        self.inner
            .call("start", &[when.into()])
            .as_::<jsbind::Undefined>()
    }

    pub fn start2(&self, when: f64, offset: f64) -> jsbind::Undefined {
        self.inner
            .call("start", &[when.into(), offset.into()])
            .as_::<jsbind::Undefined>()
    }

    pub fn start3(&self, when: f64, offset: f64, duration: f64) -> jsbind::Undefined {
        self.inner
            .call("start", &[when.into(), offset.into(), duration.into()])
            .as_::<jsbind::Undefined>()
    }
}

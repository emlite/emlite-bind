use super::*;

#[derive(Clone, Debug)]
pub struct AudioScheduledSourceNode {
    inner: AudioNode,
}
impl FromVal for AudioScheduledSourceNode {
    fn from_val(v: &emlite::Val) -> Self {
        AudioScheduledSourceNode {
            inner: AudioNode::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for AudioScheduledSourceNode {
    type Target = AudioNode;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for AudioScheduledSourceNode {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<AudioScheduledSourceNode> for emlite::Val {
    fn from(s: AudioScheduledSourceNode) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl AudioScheduledSourceNode {
    pub fn onended(&self) -> jsbind::Any {
        self.inner.get("onended").as_::<jsbind::Any>()
    }

    pub fn set_onended(&mut self, value: jsbind::Any) {
        self.inner.set("onended", value);
    }
}
impl AudioScheduledSourceNode {
    pub fn start0(&self) -> jsbind::Undefined {
        self.inner.call("start", &[]).as_::<jsbind::Undefined>()
    }

    pub fn start1(&self, when: f64) -> jsbind::Undefined {
        self.inner
            .call("start", &[when.into()])
            .as_::<jsbind::Undefined>()
    }
}
impl AudioScheduledSourceNode {
    pub fn stop0(&self) -> jsbind::Undefined {
        self.inner.call("stop", &[]).as_::<jsbind::Undefined>()
    }

    pub fn stop1(&self, when: f64) -> jsbind::Undefined {
        self.inner
            .call("stop", &[when.into()])
            .as_::<jsbind::Undefined>()
    }
}

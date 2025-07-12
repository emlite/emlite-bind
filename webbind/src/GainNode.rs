use super::*;

#[derive(Clone, Debug)]
pub struct GainNode {
    inner: AudioNode,
}
impl FromVal for GainNode {
    fn from_val(v: &emlite::Val) -> Self {
        GainNode {
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
impl std::ops::Deref for GainNode {
    type Target = AudioNode;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for GainNode {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<GainNode> for emlite::Val {
    fn from(s: GainNode) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl GainNode {
    pub fn new0(context: BaseAudioContext) -> GainNode {
        Self {
            inner: emlite::Val::global("GainNode")
                .new(&[context.into()])
                .as_::<AudioNode>(),
        }
    }

    pub fn new1(context: BaseAudioContext, options: jsbind::Any) -> GainNode {
        Self {
            inner: emlite::Val::global("GainNode")
                .new(&[context.into(), options.into()])
                .as_::<AudioNode>(),
        }
    }
}
impl GainNode {
    pub fn gain(&self) -> AudioParam {
        self.inner.get("gain").as_::<AudioParam>()
    }
}

use super::*;

#[derive(Clone, Debug)]
pub struct DelayNode {
    inner: AudioNode,
}
impl FromVal for DelayNode {
    fn from_val(v: &emlite::Val) -> Self {
        DelayNode {
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
impl std::ops::Deref for DelayNode {
    type Target = AudioNode;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for DelayNode {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<DelayNode> for emlite::Val {
    fn from(s: DelayNode) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl DelayNode {
    pub fn new0(context: BaseAudioContext) -> DelayNode {
        Self {
            inner: emlite::Val::global("DelayNode")
                .new(&[context.into()])
                .as_::<AudioNode>(),
        }
    }

    pub fn new1(context: BaseAudioContext, options: jsbind::Any) -> DelayNode {
        Self {
            inner: emlite::Val::global("DelayNode")
                .new(&[context.into(), options.into()])
                .as_::<AudioNode>(),
        }
    }
}
impl DelayNode {
    pub fn delay_time(&self) -> AudioParam {
        self.inner.get("delayTime").as_::<AudioParam>()
    }
}

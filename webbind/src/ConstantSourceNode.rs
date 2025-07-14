use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ConstantSourceNode {
    inner: AudioScheduledSourceNode,
}
impl FromVal for ConstantSourceNode {
    fn from_val(v: &emlite::Val) -> Self {
        ConstantSourceNode {
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
impl core::ops::Deref for ConstantSourceNode {
    type Target = AudioScheduledSourceNode;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for ConstantSourceNode {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<ConstantSourceNode> for emlite::Val {
    fn from(s: ConstantSourceNode) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl ConstantSourceNode {
    pub fn new0(context: BaseAudioContext) -> ConstantSourceNode {
        Self {
            inner: emlite::Val::global("ConstantSourceNode")
                .new(&[context.into()])
                .as_::<AudioScheduledSourceNode>(),
        }
    }

    pub fn new1(context: BaseAudioContext, options: jsbind::Any) -> ConstantSourceNode {
        Self {
            inner: emlite::Val::global("ConstantSourceNode")
                .new(&[context.into(), options.into()])
                .as_::<AudioScheduledSourceNode>(),
        }
    }
}
impl ConstantSourceNode {
    pub fn offset(&self) -> AudioParam {
        self.inner.get("offset").as_::<AudioParam>()
    }
}

use super::*;

#[derive(Clone, Debug)]
pub struct ChannelSplitterNode {
    inner: AudioNode,
}
impl FromVal for ChannelSplitterNode {
    fn from_val(v: &emlite::Val) -> Self {
        ChannelSplitterNode {
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
impl std::ops::Deref for ChannelSplitterNode {
    type Target = AudioNode;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for ChannelSplitterNode {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<ChannelSplitterNode> for emlite::Val {
    fn from(s: ChannelSplitterNode) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl ChannelSplitterNode {
    pub fn new0(context: BaseAudioContext) -> ChannelSplitterNode {
        Self {
            inner: emlite::Val::global("ChannelSplitterNode")
                .new(&[context.into()])
                .as_::<AudioNode>(),
        }
    }

    pub fn new1(context: BaseAudioContext, options: jsbind::Any) -> ChannelSplitterNode {
        Self {
            inner: emlite::Val::global("ChannelSplitterNode")
                .new(&[context.into(), options.into()])
                .as_::<AudioNode>(),
        }
    }
}

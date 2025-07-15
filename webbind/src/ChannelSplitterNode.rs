use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
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
impl core::ops::Deref for ChannelSplitterNode {
    type Target = AudioNode;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for ChannelSplitterNode {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for ChannelSplitterNode {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for ChannelSplitterNode {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<ChannelSplitterNode> for emlite::Val {
    fn from(s: ChannelSplitterNode) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&ChannelSplitterNode> for emlite::Val {
    fn from(s: &ChannelSplitterNode) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(ChannelSplitterNode);

impl ChannelSplitterNode {
    pub fn new0(context: &BaseAudioContext) -> ChannelSplitterNode {
        Self {
            inner: emlite::Val::global("ChannelSplitterNode")
                .new(&[context.into()])
                .as_::<AudioNode>(),
        }
    }

    pub fn new1(context: &BaseAudioContext, options: &Any) -> ChannelSplitterNode {
        Self {
            inner: emlite::Val::global("ChannelSplitterNode")
                .new(&[context.into(), options.into()])
                .as_::<AudioNode>(),
        }
    }
}

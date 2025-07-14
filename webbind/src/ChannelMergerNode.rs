use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct ChannelMergerNode {
    inner: AudioNode,
}
impl FromVal for ChannelMergerNode {
    fn from_val(v: &emlite::Val) -> Self {
        ChannelMergerNode {
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
impl core::ops::Deref for ChannelMergerNode {
    type Target = AudioNode;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for ChannelMergerNode {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for ChannelMergerNode {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for ChannelMergerNode {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<ChannelMergerNode> for emlite::Val {
    fn from(s: ChannelMergerNode) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(ChannelMergerNode);

impl ChannelMergerNode {
    pub fn new0(context: BaseAudioContext) -> ChannelMergerNode {
        Self {
            inner: emlite::Val::global("ChannelMergerNode")
                .new(&[context.into()])
                .as_::<AudioNode>(),
        }
    }

    pub fn new1(context: BaseAudioContext, options: jsbind::Any) -> ChannelMergerNode {
        Self {
            inner: emlite::Val::global("ChannelMergerNode")
                .new(&[context.into(), options.into()])
                .as_::<AudioNode>(),
        }
    }
}

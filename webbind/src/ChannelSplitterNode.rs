use super::*;

/// The ChannelSplitterNode class.
/// [`ChannelSplitterNode`](https://developer.mozilla.org/en-US/docs/Web/API/ChannelSplitterNode)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct ChannelSplitterNode {
    inner: AudioNode,
}
impl FromVal for ChannelSplitterNode {
    fn from_val(v: &Any) -> Self {
        ChannelSplitterNode {
            inner: AudioNode::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
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
impl AsRef<Any> for ChannelSplitterNode {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for ChannelSplitterNode {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<ChannelSplitterNode> for Any {
    fn from(s: ChannelSplitterNode) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&ChannelSplitterNode> for Any {
    fn from(s: &ChannelSplitterNode) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(ChannelSplitterNode);

impl ChannelSplitterNode {
    /// The `new ChannelSplitterNode(..)` constructor, creating a new ChannelSplitterNode instance
    pub fn new0(context: &BaseAudioContext) -> ChannelSplitterNode {
        Self {
            inner: Any::global("ChannelSplitterNode")
                .new(&[context.into()])
                .as_::<AudioNode>(),
        }
    }

    /// The `new ChannelSplitterNode(..)` constructor, creating a new ChannelSplitterNode instance
    pub fn new1(
        context: &BaseAudioContext,
        options: &ChannelSplitterOptions,
    ) -> ChannelSplitterNode {
        Self {
            inner: Any::global("ChannelSplitterNode")
                .new(&[context.into(), options.into()])
                .as_::<AudioNode>(),
        }
    }
}

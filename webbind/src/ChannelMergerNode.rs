use super::*;




/// The ChannelMergerNode class.
/// [`ChannelMergerNode`](https://developer.mozilla.org/en-US/docs/Web/API/ChannelMergerNode)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct ChannelMergerNode {
    inner: AudioNode,
}

impl FromVal for ChannelMergerNode {
    fn from_val(v: &Any) -> Self {
        ChannelMergerNode { inner: AudioNode::from_val(v) }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
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

impl AsRef<Any> for ChannelMergerNode {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for ChannelMergerNode {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<ChannelMergerNode> for Any {
    fn from(s: ChannelMergerNode) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&ChannelMergerNode> for Any {
    fn from(s: &ChannelMergerNode) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(ChannelMergerNode);



impl ChannelMergerNode {
    /// The `new ChannelMergerNode(..)` constructor, creating a new ChannelMergerNode instance
    pub fn new0(context: &BaseAudioContext) -> ChannelMergerNode {
        Self {
            inner: Any::global("ChannelMergerNode").new(&[context.into()]).as_::<AudioNode>(),
        }
    }

    /// The `new ChannelMergerNode(..)` constructor, creating a new ChannelMergerNode instance
    pub fn new1(context: &BaseAudioContext, options: &ChannelMergerOptions) -> ChannelMergerNode {
        Self {
            inner: Any::global("ChannelMergerNode").new(&[context.into(), options.into()]).as_::<AudioNode>(),
        }
    }

}

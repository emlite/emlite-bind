use super::*;




/// The AudioDestinationNode class.
/// [`AudioDestinationNode`](https://developer.mozilla.org/en-US/docs/Web/API/AudioDestinationNode)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct AudioDestinationNode {
    inner: AudioNode,
}

impl FromVal for AudioDestinationNode {
    fn from_val(v: &Any) -> Self {
        AudioDestinationNode { inner: AudioNode::from_val(v) }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for AudioDestinationNode {
    type Target = AudioNode;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for AudioDestinationNode {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for AudioDestinationNode {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for AudioDestinationNode {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<AudioDestinationNode> for Any {
    fn from(s: AudioDestinationNode) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&AudioDestinationNode> for Any {
    fn from(s: &AudioDestinationNode) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(AudioDestinationNode);


impl AudioDestinationNode {
    /// Getter of the `maxChannelCount` attribute.
    /// [`AudioDestinationNode.maxChannelCount`](https://developer.mozilla.org/en-US/docs/Web/API/AudioDestinationNode/maxChannelCount)
    pub fn max_channel_count(&self) -> u32 {
        self.inner.get("maxChannelCount").as_::<u32>()
    }

}

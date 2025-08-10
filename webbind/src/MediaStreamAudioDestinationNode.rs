use super::*;

/// The MediaStreamAudioDestinationNode class.
/// [`MediaStreamAudioDestinationNode`](https://developer.mozilla.org/en-US/docs/Web/API/MediaStreamAudioDestinationNode)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct MediaStreamAudioDestinationNode {
    inner: AudioNode,
}
impl FromVal for MediaStreamAudioDestinationNode {
    fn from_val(v: &Any) -> Self {
        MediaStreamAudioDestinationNode {
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
impl core::ops::Deref for MediaStreamAudioDestinationNode {
    type Target = AudioNode;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for MediaStreamAudioDestinationNode {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for MediaStreamAudioDestinationNode {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for MediaStreamAudioDestinationNode {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<MediaStreamAudioDestinationNode> for Any {
    fn from(s: MediaStreamAudioDestinationNode) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&MediaStreamAudioDestinationNode> for Any {
    fn from(s: &MediaStreamAudioDestinationNode) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(MediaStreamAudioDestinationNode);

impl MediaStreamAudioDestinationNode {
    /// The `new MediaStreamAudioDestinationNode(..)` constructor, creating a new MediaStreamAudioDestinationNode instance
    pub fn new0(context: &AudioContext) -> MediaStreamAudioDestinationNode {
        Self {
            inner: Any::global("MediaStreamAudioDestinationNode")
                .new(&[context.into()])
                .as_::<AudioNode>(),
        }
    }

    /// The `new MediaStreamAudioDestinationNode(..)` constructor, creating a new MediaStreamAudioDestinationNode instance
    pub fn new1(
        context: &AudioContext,
        options: &AudioNodeOptions,
    ) -> MediaStreamAudioDestinationNode {
        Self {
            inner: Any::global("MediaStreamAudioDestinationNode")
                .new(&[context.into(), options.into()])
                .as_::<AudioNode>(),
        }
    }
}
impl MediaStreamAudioDestinationNode {
    /// Getter of the `stream` attribute.
    /// [`MediaStreamAudioDestinationNode.stream`](https://developer.mozilla.org/en-US/docs/Web/API/MediaStreamAudioDestinationNode/stream)
    pub fn stream(&self) -> MediaStream {
        self.inner.get("stream").as_::<MediaStream>()
    }
}

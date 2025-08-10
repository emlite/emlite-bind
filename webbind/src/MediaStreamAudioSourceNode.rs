use super::*;

/// The MediaStreamAudioSourceNode class.
/// [`MediaStreamAudioSourceNode`](https://developer.mozilla.org/en-US/docs/Web/API/MediaStreamAudioSourceNode)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct MediaStreamAudioSourceNode {
    inner: AudioNode,
}
impl FromVal for MediaStreamAudioSourceNode {
    fn from_val(v: &Any) -> Self {
        MediaStreamAudioSourceNode {
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
impl core::ops::Deref for MediaStreamAudioSourceNode {
    type Target = AudioNode;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for MediaStreamAudioSourceNode {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for MediaStreamAudioSourceNode {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for MediaStreamAudioSourceNode {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<MediaStreamAudioSourceNode> for Any {
    fn from(s: MediaStreamAudioSourceNode) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&MediaStreamAudioSourceNode> for Any {
    fn from(s: &MediaStreamAudioSourceNode) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(MediaStreamAudioSourceNode);

impl MediaStreamAudioSourceNode {
    /// The `new MediaStreamAudioSourceNode(..)` constructor, creating a new MediaStreamAudioSourceNode instance
    pub fn new(
        context: &AudioContext,
        options: &MediaStreamAudioSourceOptions,
    ) -> MediaStreamAudioSourceNode {
        Self {
            inner: Any::global("MediaStreamAudioSourceNode")
                .new(&[context.into(), options.into()])
                .as_::<AudioNode>(),
        }
    }
}
impl MediaStreamAudioSourceNode {
    /// Getter of the `mediaStream` attribute.
    /// [`MediaStreamAudioSourceNode.mediaStream`](https://developer.mozilla.org/en-US/docs/Web/API/MediaStreamAudioSourceNode/mediaStream)
    pub fn media_stream(&self) -> MediaStream {
        self.inner.get("mediaStream").as_::<MediaStream>()
    }
}

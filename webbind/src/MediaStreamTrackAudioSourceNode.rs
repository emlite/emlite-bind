use super::*;

/// The MediaStreamTrackAudioSourceNode class.
/// [`MediaStreamTrackAudioSourceNode`](https://developer.mozilla.org/en-US/docs/Web/API/MediaStreamTrackAudioSourceNode)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct MediaStreamTrackAudioSourceNode {
    inner: AudioNode,
}
impl FromVal for MediaStreamTrackAudioSourceNode {
    fn from_val(v: &Any) -> Self {
        MediaStreamTrackAudioSourceNode {
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
impl core::ops::Deref for MediaStreamTrackAudioSourceNode {
    type Target = AudioNode;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for MediaStreamTrackAudioSourceNode {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for MediaStreamTrackAudioSourceNode {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for MediaStreamTrackAudioSourceNode {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<MediaStreamTrackAudioSourceNode> for Any {
    fn from(s: MediaStreamTrackAudioSourceNode) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&MediaStreamTrackAudioSourceNode> for Any {
    fn from(s: &MediaStreamTrackAudioSourceNode) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(MediaStreamTrackAudioSourceNode);

impl MediaStreamTrackAudioSourceNode {
    /// The `new MediaStreamTrackAudioSourceNode(..)` constructor, creating a new MediaStreamTrackAudioSourceNode instance
    pub fn new(
        context: &AudioContext,
        options: &MediaStreamTrackAudioSourceOptions,
    ) -> MediaStreamTrackAudioSourceNode {
        Self {
            inner: Any::global("MediaStreamTrackAudioSourceNode")
                .new(&[context.into(), options.into()])
                .as_::<AudioNode>(),
        }
    }
}

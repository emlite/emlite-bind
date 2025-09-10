use super::*;

/// The MediaElementAudioSourceNode class.
/// [`MediaElementAudioSourceNode`](https://developer.mozilla.org/en-US/docs/Web/API/MediaElementAudioSourceNode)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct MediaElementAudioSourceNode {
    inner: AudioNode,
}

impl FromVal for MediaElementAudioSourceNode {
    fn from_val(v: &Any) -> Self {
        MediaElementAudioSourceNode {
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

impl core::ops::Deref for MediaElementAudioSourceNode {
    type Target = AudioNode;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for MediaElementAudioSourceNode {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for MediaElementAudioSourceNode {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for MediaElementAudioSourceNode {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<MediaElementAudioSourceNode> for Any {
    fn from(s: MediaElementAudioSourceNode) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&MediaElementAudioSourceNode> for Any {
    fn from(s: &MediaElementAudioSourceNode) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(MediaElementAudioSourceNode);

impl MediaElementAudioSourceNode {
    /// The `new MediaElementAudioSourceNode(..)` constructor, creating a new MediaElementAudioSourceNode instance
    pub fn new(
        context: &AudioContext,
        options: &MediaElementAudioSourceOptions,
    ) -> MediaElementAudioSourceNode {
        Self {
            inner: Any::global("MediaElementAudioSourceNode")
                .new(&[context.into(), options.into()])
                .as_::<AudioNode>(),
        }
    }
}
impl MediaElementAudioSourceNode {
    /// Getter of the `mediaElement` attribute.
    /// [`MediaElementAudioSourceNode.mediaElement`](https://developer.mozilla.org/en-US/docs/Web/API/MediaElementAudioSourceNode/mediaElement)
    pub fn media_element(&self) -> HTMLMediaElement {
        self.inner.get("mediaElement").as_::<HTMLMediaElement>()
    }
}

use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MediaStreamAudioDestinationNode {
    inner: AudioNode,
}
impl FromVal for MediaStreamAudioDestinationNode {
    fn from_val(v: &emlite::Val) -> Self {
        MediaStreamAudioDestinationNode {
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
impl From<MediaStreamAudioDestinationNode> for emlite::Val {
    fn from(s: MediaStreamAudioDestinationNode) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl MediaStreamAudioDestinationNode {
    pub fn new0(context: AudioContext) -> MediaStreamAudioDestinationNode {
        Self {
            inner: emlite::Val::global("MediaStreamAudioDestinationNode")
                .new(&[context.into()])
                .as_::<AudioNode>(),
        }
    }

    pub fn new1(context: AudioContext, options: jsbind::Any) -> MediaStreamAudioDestinationNode {
        Self {
            inner: emlite::Val::global("MediaStreamAudioDestinationNode")
                .new(&[context.into(), options.into()])
                .as_::<AudioNode>(),
        }
    }
}
impl MediaStreamAudioDestinationNode {
    pub fn stream(&self) -> MediaStream {
        self.inner.get("stream").as_::<MediaStream>()
    }
}

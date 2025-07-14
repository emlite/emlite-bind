use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MediaStreamAudioSourceNode {
    inner: AudioNode,
}
impl FromVal for MediaStreamAudioSourceNode {
    fn from_val(v: &emlite::Val) -> Self {
        MediaStreamAudioSourceNode {
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
impl From<MediaStreamAudioSourceNode> for emlite::Val {
    fn from(s: MediaStreamAudioSourceNode) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl MediaStreamAudioSourceNode {
    pub fn new(context: AudioContext, options: jsbind::Any) -> MediaStreamAudioSourceNode {
        Self {
            inner: emlite::Val::global("MediaStreamAudioSourceNode")
                .new(&[context.into(), options.into()])
                .as_::<AudioNode>(),
        }
    }
}
impl MediaStreamAudioSourceNode {
    pub fn media_stream(&self) -> MediaStream {
        self.inner.get("mediaStream").as_::<MediaStream>()
    }
}

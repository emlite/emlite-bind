use super::*;

#[derive(Clone, Debug)]
pub struct MediaElementAudioSourceNode {
    inner: AudioNode,
}
impl FromVal for MediaElementAudioSourceNode {
    fn from_val(v: &emlite::Val) -> Self {
        MediaElementAudioSourceNode {
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
impl std::ops::Deref for MediaElementAudioSourceNode {
    type Target = AudioNode;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for MediaElementAudioSourceNode {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<MediaElementAudioSourceNode> for emlite::Val {
    fn from(s: MediaElementAudioSourceNode) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl MediaElementAudioSourceNode {
    pub fn new(context: AudioContext, options: jsbind::Any) -> MediaElementAudioSourceNode {
        Self {
            inner: emlite::Val::global("MediaElementAudioSourceNode")
                .new(&[context.into(), options.into()])
                .as_::<AudioNode>(),
        }
    }
}
impl MediaElementAudioSourceNode {
    pub fn media_element(&self) -> HTMLMediaElement {
        self.inner.get("mediaElement").as_::<HTMLMediaElement>()
    }
}

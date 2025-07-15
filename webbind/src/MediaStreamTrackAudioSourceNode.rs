use super::*;




#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct MediaStreamTrackAudioSourceNode {
    inner: AudioNode,
}
impl FromVal for MediaStreamTrackAudioSourceNode {
    fn from_val(v: &emlite::Val) -> Self {
        MediaStreamTrackAudioSourceNode { inner: AudioNode::from_val(v) }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
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
impl AsRef<emlite::Val> for MediaStreamTrackAudioSourceNode {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for MediaStreamTrackAudioSourceNode {
    fn as_mut(&mut self) -> &mut emlite::Val {
      &mut self.inner
  }
}
impl From<MediaStreamTrackAudioSourceNode> for emlite::Val {
    fn from(s: MediaStreamTrackAudioSourceNode) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(MediaStreamTrackAudioSourceNode);



impl MediaStreamTrackAudioSourceNode {
    pub fn new(context: AudioContext, options: Any) -> MediaStreamTrackAudioSourceNode {
        Self {
            inner: emlite::Val::global("MediaStreamTrackAudioSourceNode").new(&[context.into(), options.into()]).as_::<AudioNode>(),
        }
    }

}

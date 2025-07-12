use super::*;

#[derive(Clone, Debug)]
pub struct StereoPannerNode {
    inner: AudioNode,
}
impl FromVal for StereoPannerNode {
    fn from_val(v: &emlite::Val) -> Self {
        StereoPannerNode {
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
impl std::ops::Deref for StereoPannerNode {
    type Target = AudioNode;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for StereoPannerNode {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<StereoPannerNode> for emlite::Val {
    fn from(s: StereoPannerNode) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl StereoPannerNode {
    pub fn new0(context: BaseAudioContext) -> StereoPannerNode {
        Self {
            inner: emlite::Val::global("StereoPannerNode")
                .new(&[context.into()])
                .as_::<AudioNode>(),
        }
    }

    pub fn new1(context: BaseAudioContext, options: jsbind::Any) -> StereoPannerNode {
        Self {
            inner: emlite::Val::global("StereoPannerNode")
                .new(&[context.into(), options.into()])
                .as_::<AudioNode>(),
        }
    }
}
impl StereoPannerNode {
    pub fn pan(&self) -> AudioParam {
        self.inner.get("pan").as_::<AudioParam>()
    }
}

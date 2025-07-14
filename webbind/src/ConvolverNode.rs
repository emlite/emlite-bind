use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct ConvolverNode {
    inner: AudioNode,
}
impl FromVal for ConvolverNode {
    fn from_val(v: &emlite::Val) -> Self {
        ConvolverNode {
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
impl core::ops::Deref for ConvolverNode {
    type Target = AudioNode;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for ConvolverNode {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for ConvolverNode {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for ConvolverNode {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<ConvolverNode> for emlite::Val {
    fn from(s: ConvolverNode) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(ConvolverNode);

impl ConvolverNode {
    pub fn new0(context: BaseAudioContext) -> ConvolverNode {
        Self {
            inner: emlite::Val::global("ConvolverNode")
                .new(&[context.into()])
                .as_::<AudioNode>(),
        }
    }

    pub fn new1(context: BaseAudioContext, options: jsbind::Any) -> ConvolverNode {
        Self {
            inner: emlite::Val::global("ConvolverNode")
                .new(&[context.into(), options.into()])
                .as_::<AudioNode>(),
        }
    }
}
impl ConvolverNode {
    pub fn buffer(&self) -> AudioBuffer {
        self.inner.get("buffer").as_::<AudioBuffer>()
    }

    pub fn set_buffer(&mut self, value: AudioBuffer) {
        self.inner.set("buffer", value);
    }
}
impl ConvolverNode {
    pub fn normalize(&self) -> bool {
        self.inner.get("normalize").as_::<bool>()
    }

    pub fn set_normalize(&mut self, value: bool) {
        self.inner.set("normalize", value);
    }
}

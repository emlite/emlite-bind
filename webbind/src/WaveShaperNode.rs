use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct WaveShaperNode {
    inner: AudioNode,
}
impl FromVal for WaveShaperNode {
    fn from_val(v: &emlite::Val) -> Self {
        WaveShaperNode {
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
impl core::ops::Deref for WaveShaperNode {
    type Target = AudioNode;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for WaveShaperNode {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for WaveShaperNode {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for WaveShaperNode {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<WaveShaperNode> for emlite::Val {
    fn from(s: WaveShaperNode) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&WaveShaperNode> for emlite::Val {
    fn from(s: &WaveShaperNode) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(WaveShaperNode);

impl WaveShaperNode {
    pub fn new0(context: &BaseAudioContext) -> WaveShaperNode {
        Self {
            inner: emlite::Val::global("WaveShaperNode")
                .new(&[context.into()])
                .as_::<AudioNode>(),
        }
    }

    pub fn new1(context: &BaseAudioContext, options: &Any) -> WaveShaperNode {
        Self {
            inner: emlite::Val::global("WaveShaperNode")
                .new(&[context.into(), options.into()])
                .as_::<AudioNode>(),
        }
    }
}
impl WaveShaperNode {
    pub fn curve(&self) -> Float32Array {
        self.inner.get("curve").as_::<Float32Array>()
    }

    pub fn set_curve(&mut self, value: &Float32Array) {
        self.inner.set("curve", value);
    }
}
impl WaveShaperNode {
    pub fn oversample(&self) -> OverSampleType {
        self.inner.get("oversample").as_::<OverSampleType>()
    }

    pub fn set_oversample(&mut self, value: &OverSampleType) {
        self.inner.set("oversample", value);
    }
}

use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct DynamicsCompressorNode {
    inner: AudioNode,
}
impl FromVal for DynamicsCompressorNode {
    fn from_val(v: &emlite::Val) -> Self {
        DynamicsCompressorNode {
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
impl core::ops::Deref for DynamicsCompressorNode {
    type Target = AudioNode;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for DynamicsCompressorNode {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for DynamicsCompressorNode {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for DynamicsCompressorNode {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<DynamicsCompressorNode> for emlite::Val {
    fn from(s: DynamicsCompressorNode) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(DynamicsCompressorNode);

impl DynamicsCompressorNode {
    pub fn new0(context: BaseAudioContext) -> DynamicsCompressorNode {
        Self {
            inner: emlite::Val::global("DynamicsCompressorNode")
                .new(&[context.into()])
                .as_::<AudioNode>(),
        }
    }

    pub fn new1(context: BaseAudioContext, options: jsbind::Any) -> DynamicsCompressorNode {
        Self {
            inner: emlite::Val::global("DynamicsCompressorNode")
                .new(&[context.into(), options.into()])
                .as_::<AudioNode>(),
        }
    }
}
impl DynamicsCompressorNode {
    pub fn threshold(&self) -> AudioParam {
        self.inner.get("threshold").as_::<AudioParam>()
    }
}
impl DynamicsCompressorNode {
    pub fn knee(&self) -> AudioParam {
        self.inner.get("knee").as_::<AudioParam>()
    }
}
impl DynamicsCompressorNode {
    pub fn ratio(&self) -> AudioParam {
        self.inner.get("ratio").as_::<AudioParam>()
    }
}
impl DynamicsCompressorNode {
    pub fn reduction(&self) -> f32 {
        self.inner.get("reduction").as_::<f32>()
    }
}
impl DynamicsCompressorNode {
    pub fn attack(&self) -> AudioParam {
        self.inner.get("attack").as_::<AudioParam>()
    }
}
impl DynamicsCompressorNode {
    pub fn release(&self) -> AudioParam {
        self.inner.get("release").as_::<AudioParam>()
    }
}

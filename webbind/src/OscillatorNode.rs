use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct OscillatorNode {
    inner: AudioScheduledSourceNode,
}
impl FromVal for OscillatorNode {
    fn from_val(v: &emlite::Val) -> Self {
        OscillatorNode {
            inner: AudioScheduledSourceNode::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for OscillatorNode {
    type Target = AudioScheduledSourceNode;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for OscillatorNode {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for OscillatorNode {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for OscillatorNode {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<OscillatorNode> for emlite::Val {
    fn from(s: OscillatorNode) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&OscillatorNode> for emlite::Val {
    fn from(s: &OscillatorNode) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(OscillatorNode);

impl OscillatorNode {
    pub fn new0(context: BaseAudioContext) -> OscillatorNode {
        Self {
            inner: emlite::Val::global("OscillatorNode")
                .new(&[context.into()])
                .as_::<AudioScheduledSourceNode>(),
        }
    }

    pub fn new1(context: BaseAudioContext, options: Any) -> OscillatorNode {
        Self {
            inner: emlite::Val::global("OscillatorNode")
                .new(&[context.into(), options.into()])
                .as_::<AudioScheduledSourceNode>(),
        }
    }
}
impl OscillatorNode {
    pub fn type_(&self) -> OscillatorType {
        self.inner.get("type").as_::<OscillatorType>()
    }

    pub fn set_type_(&mut self, value: OscillatorType) {
        self.inner.set("type", value);
    }
}
impl OscillatorNode {
    pub fn frequency(&self) -> AudioParam {
        self.inner.get("frequency").as_::<AudioParam>()
    }
}
impl OscillatorNode {
    pub fn detune(&self) -> AudioParam {
        self.inner.get("detune").as_::<AudioParam>()
    }
}
impl OscillatorNode {
    pub fn set_periodic_wave(&self, periodic_wave: PeriodicWave) -> Undefined {
        self.inner
            .call("setPeriodicWave", &[periodic_wave.into()])
            .as_::<Undefined>()
    }
}

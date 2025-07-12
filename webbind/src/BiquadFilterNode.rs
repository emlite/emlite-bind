use super::*;

#[derive(Clone, Debug)]
pub struct BiquadFilterNode {
    inner: AudioNode,
}
impl FromVal for BiquadFilterNode {
    fn from_val(v: &emlite::Val) -> Self {
        BiquadFilterNode {
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
impl std::ops::Deref for BiquadFilterNode {
    type Target = AudioNode;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for BiquadFilterNode {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<BiquadFilterNode> for emlite::Val {
    fn from(s: BiquadFilterNode) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl BiquadFilterNode {
    pub fn new0(context: BaseAudioContext) -> BiquadFilterNode {
        Self {
            inner: emlite::Val::global("BiquadFilterNode")
                .new(&[context.into()])
                .as_::<AudioNode>(),
        }
    }

    pub fn new1(context: BaseAudioContext, options: jsbind::Any) -> BiquadFilterNode {
        Self {
            inner: emlite::Val::global("BiquadFilterNode")
                .new(&[context.into(), options.into()])
                .as_::<AudioNode>(),
        }
    }
}
impl BiquadFilterNode {
    pub fn type_(&self) -> BiquadFilterType {
        self.inner.get("type").as_::<BiquadFilterType>()
    }

    pub fn set_type_(&mut self, value: BiquadFilterType) {
        self.inner.set("type", value);
    }
}
impl BiquadFilterNode {
    pub fn frequency(&self) -> AudioParam {
        self.inner.get("frequency").as_::<AudioParam>()
    }
}
impl BiquadFilterNode {
    pub fn detune(&self) -> AudioParam {
        self.inner.get("detune").as_::<AudioParam>()
    }
}
impl BiquadFilterNode {
    pub fn q(&self) -> AudioParam {
        self.inner.get("Q").as_::<AudioParam>()
    }
}
impl BiquadFilterNode {
    pub fn gain(&self) -> AudioParam {
        self.inner.get("gain").as_::<AudioParam>()
    }
}
impl BiquadFilterNode {
    pub fn get_frequency_response(
        &self,
        frequency_hz: jsbind::Float32Array,
        mag_response: jsbind::Float32Array,
        phase_response: jsbind::Float32Array,
    ) -> jsbind::Undefined {
        self.inner
            .call(
                "getFrequencyResponse",
                &[
                    frequency_hz.into(),
                    mag_response.into(),
                    phase_response.into(),
                ],
            )
            .as_::<jsbind::Undefined>()
    }
}

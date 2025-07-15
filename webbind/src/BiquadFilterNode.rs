use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
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
impl core::ops::Deref for BiquadFilterNode {
    type Target = AudioNode;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for BiquadFilterNode {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for BiquadFilterNode {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for BiquadFilterNode {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<BiquadFilterNode> for emlite::Val {
    fn from(s: BiquadFilterNode) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&BiquadFilterNode> for emlite::Val {
    fn from(s: &BiquadFilterNode) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(BiquadFilterNode);

impl BiquadFilterNode {
    pub fn new0(context: &BaseAudioContext) -> BiquadFilterNode {
        Self {
            inner: emlite::Val::global("BiquadFilterNode")
                .new(&[context.into()])
                .as_::<AudioNode>(),
        }
    }

    pub fn new1(context: &BaseAudioContext, options: &Any) -> BiquadFilterNode {
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

    pub fn set_type_(&mut self, value: &BiquadFilterType) {
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
        frequency_hz: &Float32Array,
        mag_response: &Float32Array,
        phase_response: &Float32Array,
    ) -> Undefined {
        self.inner
            .call(
                "getFrequencyResponse",
                &[
                    frequency_hz.into(),
                    mag_response.into(),
                    phase_response.into(),
                ],
            )
            .as_::<Undefined>()
    }
}

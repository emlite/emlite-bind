use super::*;

#[derive(Clone, Debug)]
pub struct IIRFilterNode {
    inner: AudioNode,
}
impl FromVal for IIRFilterNode {
    fn from_val(v: &emlite::Val) -> Self {
        IIRFilterNode {
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
impl std::ops::Deref for IIRFilterNode {
    type Target = AudioNode;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for IIRFilterNode {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<IIRFilterNode> for emlite::Val {
    fn from(s: IIRFilterNode) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl IIRFilterNode {
    pub fn new(context: BaseAudioContext, options: jsbind::Any) -> IIRFilterNode {
        Self {
            inner: emlite::Val::global("IIRFilterNode")
                .new(&[context.into(), options.into()])
                .as_::<AudioNode>(),
        }
    }
}
impl IIRFilterNode {
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

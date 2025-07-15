use super::*;




#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct IIRFilterNode {
    inner: AudioNode,
}
impl FromVal for IIRFilterNode {
    fn from_val(v: &emlite::Val) -> Self {
        IIRFilterNode { inner: AudioNode::from_val(v) }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for IIRFilterNode {
    type Target = AudioNode;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for IIRFilterNode {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for IIRFilterNode {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for IIRFilterNode {
    fn as_mut(&mut self) -> &mut emlite::Val {
      &mut self.inner
  }
}
impl From<IIRFilterNode> for emlite::Val {
    fn from(s: IIRFilterNode) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(IIRFilterNode);



impl IIRFilterNode {
    pub fn new(context: BaseAudioContext, options: Any) -> IIRFilterNode {
        Self {
            inner: emlite::Val::global("IIRFilterNode").new(&[context.into(), options.into()]).as_::<AudioNode>(),
        }
    }

}
impl IIRFilterNode {
    pub fn get_frequency_response(&self, frequency_hz: Float32Array, mag_response: Float32Array, phase_response: Float32Array) -> Undefined {
        self.inner.call("getFrequencyResponse", &[frequency_hz.into(), mag_response.into(), phase_response.into(), ]).as_::<Undefined>()
    }

}

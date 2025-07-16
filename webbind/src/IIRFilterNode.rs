use super::*;

/// The IIRFilterNode class.
/// [`IIRFilterNode`](https://developer.mozilla.org/en-US/docs/Web/API/IIRFilterNode)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct IIRFilterNode {
    inner: AudioNode,
}
impl FromVal for IIRFilterNode {
    fn from_val(v: &Any) -> Self {
        IIRFilterNode {
            inner: AudioNode::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
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
impl AsRef<Any> for IIRFilterNode {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for IIRFilterNode {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<IIRFilterNode> for Any {
    fn from(s: IIRFilterNode) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&IIRFilterNode> for Any {
    fn from(s: &IIRFilterNode) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(IIRFilterNode);

impl IIRFilterNode {
    /// The `new IIRFilterNode(..)` constructor, creating a new IIRFilterNode instance
    pub fn new(context: &BaseAudioContext, options: &Any) -> IIRFilterNode {
        Self {
            inner: Any::global("IIRFilterNode")
                .new(&[context.into(), options.into()])
                .as_::<AudioNode>(),
        }
    }
}
impl IIRFilterNode {
    /// The getFrequencyResponse method.
    /// [`IIRFilterNode.getFrequencyResponse`](https://developer.mozilla.org/en-US/docs/Web/API/IIRFilterNode/getFrequencyResponse)
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

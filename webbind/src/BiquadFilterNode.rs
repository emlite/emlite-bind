use super::*;

/// The BiquadFilterNode class.
/// [`BiquadFilterNode`](https://developer.mozilla.org/en-US/docs/Web/API/BiquadFilterNode)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct BiquadFilterNode {
    inner: AudioNode,
}

impl FromVal for BiquadFilterNode {
    fn from_val(v: &Any) -> Self {
        BiquadFilterNode {
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

impl AsRef<Any> for BiquadFilterNode {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for BiquadFilterNode {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<BiquadFilterNode> for Any {
    fn from(s: BiquadFilterNode) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&BiquadFilterNode> for Any {
    fn from(s: &BiquadFilterNode) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(BiquadFilterNode);

impl BiquadFilterNode {
    /// Getter of the `type` attribute.
    /// [`BiquadFilterNode.type`](https://developer.mozilla.org/en-US/docs/Web/API/BiquadFilterNode/type)
    pub fn type_(&self) -> BiquadFilterType {
        self.inner.get("type").as_::<BiquadFilterType>()
    }

    /// Setter of the `type` attribute.
    /// [`BiquadFilterNode.type`](https://developer.mozilla.org/en-US/docs/Web/API/BiquadFilterNode/type)
    pub fn set_type_(&mut self, value: &BiquadFilterType) {
        self.inner.set("type", value);
    }
}
impl BiquadFilterNode {
    /// Getter of the `frequency` attribute.
    /// [`BiquadFilterNode.frequency`](https://developer.mozilla.org/en-US/docs/Web/API/BiquadFilterNode/frequency)
    pub fn frequency(&self) -> AudioParam {
        self.inner.get("frequency").as_::<AudioParam>()
    }
}
impl BiquadFilterNode {
    /// Getter of the `detune` attribute.
    /// [`BiquadFilterNode.detune`](https://developer.mozilla.org/en-US/docs/Web/API/BiquadFilterNode/detune)
    pub fn detune(&self) -> AudioParam {
        self.inner.get("detune").as_::<AudioParam>()
    }
}
impl BiquadFilterNode {
    /// Getter of the `Q` attribute.
    /// [`BiquadFilterNode.Q`](https://developer.mozilla.org/en-US/docs/Web/API/BiquadFilterNode/Q)
    pub fn q(&self) -> AudioParam {
        self.inner.get("Q").as_::<AudioParam>()
    }
}
impl BiquadFilterNode {
    /// Getter of the `gain` attribute.
    /// [`BiquadFilterNode.gain`](https://developer.mozilla.org/en-US/docs/Web/API/BiquadFilterNode/gain)
    pub fn gain(&self) -> AudioParam {
        self.inner.get("gain").as_::<AudioParam>()
    }
}

impl BiquadFilterNode {
    /// The `new BiquadFilterNode(..)` constructor, creating a new BiquadFilterNode instance
    pub fn new(context: &BaseAudioContext) -> BiquadFilterNode {
        Self {
            inner: Any::global("BiquadFilterNode")
                .new(&[context.into()])
                .as_::<AudioNode>(),
        }
    }
}

impl BiquadFilterNode {
    /// The `new BiquadFilterNode(..)` constructor, creating a new BiquadFilterNode instance
    pub fn new_with_options(
        context: &BaseAudioContext,
        options: &BiquadFilterOptions,
    ) -> BiquadFilterNode {
        Self {
            inner: Any::global("BiquadFilterNode")
                .new(&[context.into(), options.into()])
                .as_::<AudioNode>(),
        }
    }
}

impl BiquadFilterNode {
    /// The getFrequencyResponse method.
    /// [`BiquadFilterNode.getFrequencyResponse`](https://developer.mozilla.org/en-US/docs/Web/API/BiquadFilterNode/getFrequencyResponse)
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

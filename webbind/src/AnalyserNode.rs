use super::*;

/// The AnalyserNode class.
/// [`AnalyserNode`](https://developer.mozilla.org/en-US/docs/Web/API/AnalyserNode)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct AnalyserNode {
    inner: AudioNode,
}

impl FromVal for AnalyserNode {
    fn from_val(v: &Any) -> Self {
        AnalyserNode {
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

impl core::ops::Deref for AnalyserNode {
    type Target = AudioNode;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for AnalyserNode {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for AnalyserNode {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for AnalyserNode {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<AnalyserNode> for Any {
    fn from(s: AnalyserNode) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&AnalyserNode> for Any {
    fn from(s: &AnalyserNode) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(AnalyserNode);

impl AnalyserNode {
    /// Getter of the `fftSize` attribute.
    /// [`AnalyserNode.fftSize`](https://developer.mozilla.org/en-US/docs/Web/API/AnalyserNode/fftSize)
    pub fn fft_size(&self) -> u32 {
        self.inner.get("fftSize").as_::<u32>()
    }

    /// Setter of the `fftSize` attribute.
    /// [`AnalyserNode.fftSize`](https://developer.mozilla.org/en-US/docs/Web/API/AnalyserNode/fftSize)
    pub fn set_fft_size(&mut self, value: u32) {
        self.inner.set("fftSize", value);
    }
}
impl AnalyserNode {
    /// Getter of the `frequencyBinCount` attribute.
    /// [`AnalyserNode.frequencyBinCount`](https://developer.mozilla.org/en-US/docs/Web/API/AnalyserNode/frequencyBinCount)
    pub fn frequency_bin_count(&self) -> u32 {
        self.inner.get("frequencyBinCount").as_::<u32>()
    }
}
impl AnalyserNode {
    /// Getter of the `minDecibels` attribute.
    /// [`AnalyserNode.minDecibels`](https://developer.mozilla.org/en-US/docs/Web/API/AnalyserNode/minDecibels)
    pub fn min_decibels(&self) -> f64 {
        self.inner.get("minDecibels").as_::<f64>()
    }

    /// Setter of the `minDecibels` attribute.
    /// [`AnalyserNode.minDecibels`](https://developer.mozilla.org/en-US/docs/Web/API/AnalyserNode/minDecibels)
    pub fn set_min_decibels(&mut self, value: f64) {
        self.inner.set("minDecibels", value);
    }
}
impl AnalyserNode {
    /// Getter of the `maxDecibels` attribute.
    /// [`AnalyserNode.maxDecibels`](https://developer.mozilla.org/en-US/docs/Web/API/AnalyserNode/maxDecibels)
    pub fn max_decibels(&self) -> f64 {
        self.inner.get("maxDecibels").as_::<f64>()
    }

    /// Setter of the `maxDecibels` attribute.
    /// [`AnalyserNode.maxDecibels`](https://developer.mozilla.org/en-US/docs/Web/API/AnalyserNode/maxDecibels)
    pub fn set_max_decibels(&mut self, value: f64) {
        self.inner.set("maxDecibels", value);
    }
}
impl AnalyserNode {
    /// Getter of the `smoothingTimeConstant` attribute.
    /// [`AnalyserNode.smoothingTimeConstant`](https://developer.mozilla.org/en-US/docs/Web/API/AnalyserNode/smoothingTimeConstant)
    pub fn smoothing_time_constant(&self) -> f64 {
        self.inner.get("smoothingTimeConstant").as_::<f64>()
    }

    /// Setter of the `smoothingTimeConstant` attribute.
    /// [`AnalyserNode.smoothingTimeConstant`](https://developer.mozilla.org/en-US/docs/Web/API/AnalyserNode/smoothingTimeConstant)
    pub fn set_smoothing_time_constant(&mut self, value: f64) {
        self.inner.set("smoothingTimeConstant", value);
    }
}

impl AnalyserNode {
    /// The `new AnalyserNode(..)` constructor, creating a new AnalyserNode instance
    pub fn new(context: &BaseAudioContext) -> AnalyserNode {
        Self {
            inner: Any::global("AnalyserNode")
                .new(&[context.into()])
                .as_::<AudioNode>(),
        }
    }
}

impl AnalyserNode {
    /// The `new AnalyserNode(..)` constructor, creating a new AnalyserNode instance
    pub fn new_with_options(context: &BaseAudioContext, options: &AnalyserOptions) -> AnalyserNode {
        Self {
            inner: Any::global("AnalyserNode")
                .new(&[context.into(), options.into()])
                .as_::<AudioNode>(),
        }
    }
}

impl AnalyserNode {
    /// The getFloatFrequencyData method.
    /// [`AnalyserNode.getFloatFrequencyData`](https://developer.mozilla.org/en-US/docs/Web/API/AnalyserNode/getFloatFrequencyData)
    pub fn get_float_frequency_data(&self, array: &Float32Array) -> Undefined {
        self.inner
            .call("getFloatFrequencyData", &[array.into()])
            .as_::<Undefined>()
    }
}
impl AnalyserNode {
    /// The getByteFrequencyData method.
    /// [`AnalyserNode.getByteFrequencyData`](https://developer.mozilla.org/en-US/docs/Web/API/AnalyserNode/getByteFrequencyData)
    pub fn get_byte_frequency_data(&self, array: &Uint8Array) -> Undefined {
        self.inner
            .call("getByteFrequencyData", &[array.into()])
            .as_::<Undefined>()
    }
}
impl AnalyserNode {
    /// The getFloatTimeDomainData method.
    /// [`AnalyserNode.getFloatTimeDomainData`](https://developer.mozilla.org/en-US/docs/Web/API/AnalyserNode/getFloatTimeDomainData)
    pub fn get_float_time_domain_data(&self, array: &Float32Array) -> Undefined {
        self.inner
            .call("getFloatTimeDomainData", &[array.into()])
            .as_::<Undefined>()
    }
}
impl AnalyserNode {
    /// The getByteTimeDomainData method.
    /// [`AnalyserNode.getByteTimeDomainData`](https://developer.mozilla.org/en-US/docs/Web/API/AnalyserNode/getByteTimeDomainData)
    pub fn get_byte_time_domain_data(&self, array: &Uint8Array) -> Undefined {
        self.inner
            .call("getByteTimeDomainData", &[array.into()])
            .as_::<Undefined>()
    }
}

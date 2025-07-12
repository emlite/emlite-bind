use super::*;

#[derive(Clone, Debug)]
pub struct AnalyserNode {
    inner: AudioNode,
}
impl FromVal for AnalyserNode {
    fn from_val(v: &emlite::Val) -> Self {
        AnalyserNode {
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
impl std::ops::Deref for AnalyserNode {
    type Target = AudioNode;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for AnalyserNode {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<AnalyserNode> for emlite::Val {
    fn from(s: AnalyserNode) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl AnalyserNode {
    pub fn new0(context: BaseAudioContext) -> AnalyserNode {
        Self {
            inner: emlite::Val::global("AnalyserNode")
                .new(&[context.into()])
                .as_::<AudioNode>(),
        }
    }

    pub fn new1(context: BaseAudioContext, options: jsbind::Any) -> AnalyserNode {
        Self {
            inner: emlite::Val::global("AnalyserNode")
                .new(&[context.into(), options.into()])
                .as_::<AudioNode>(),
        }
    }
}
impl AnalyserNode {
    pub fn get_float_frequency_data(&self, array: jsbind::Float32Array) -> jsbind::Undefined {
        self.inner
            .call("getFloatFrequencyData", &[array.into()])
            .as_::<jsbind::Undefined>()
    }
}
impl AnalyserNode {
    pub fn get_byte_frequency_data(&self, array: jsbind::Uint8Array) -> jsbind::Undefined {
        self.inner
            .call("getByteFrequencyData", &[array.into()])
            .as_::<jsbind::Undefined>()
    }
}
impl AnalyserNode {
    pub fn get_float_time_domain_data(&self, array: jsbind::Float32Array) -> jsbind::Undefined {
        self.inner
            .call("getFloatTimeDomainData", &[array.into()])
            .as_::<jsbind::Undefined>()
    }
}
impl AnalyserNode {
    pub fn get_byte_time_domain_data(&self, array: jsbind::Uint8Array) -> jsbind::Undefined {
        self.inner
            .call("getByteTimeDomainData", &[array.into()])
            .as_::<jsbind::Undefined>()
    }
}
impl AnalyserNode {
    pub fn fft_size(&self) -> u32 {
        self.inner.get("fftSize").as_::<u32>()
    }

    pub fn set_fft_size(&mut self, value: u32) {
        self.inner.set("fftSize", value);
    }
}
impl AnalyserNode {
    pub fn frequency_bin_count(&self) -> u32 {
        self.inner.get("frequencyBinCount").as_::<u32>()
    }
}
impl AnalyserNode {
    pub fn min_decibels(&self) -> f64 {
        self.inner.get("minDecibels").as_::<f64>()
    }

    pub fn set_min_decibels(&mut self, value: f64) {
        self.inner.set("minDecibels", value);
    }
}
impl AnalyserNode {
    pub fn max_decibels(&self) -> f64 {
        self.inner.get("maxDecibels").as_::<f64>()
    }

    pub fn set_max_decibels(&mut self, value: f64) {
        self.inner.set("maxDecibels", value);
    }
}
impl AnalyserNode {
    pub fn smoothing_time_constant(&self) -> f64 {
        self.inner.get("smoothingTimeConstant").as_::<f64>()
    }

    pub fn set_smoothing_time_constant(&mut self, value: f64) {
        self.inner.set("smoothingTimeConstant", value);
    }
}

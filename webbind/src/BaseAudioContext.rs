use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct PeriodicWaveConstraints {
    inner: emlite::Val,
}
impl FromVal for PeriodicWaveConstraints {
    fn from_val(v: &emlite::Val) -> Self {
        PeriodicWaveConstraints { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for PeriodicWaveConstraints {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for PeriodicWaveConstraints {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<PeriodicWaveConstraints> for emlite::Val {
    fn from(s: PeriodicWaveConstraints) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl PeriodicWaveConstraints {
    pub fn disable_normalization(&self) -> bool {
        self.inner.get("disableNormalization").as_::<bool>()
    }

    pub fn set_disable_normalization(&mut self, value: bool) {
        self.inner.set("disableNormalization", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct BaseAudioContext {
    inner: EventTarget,
}
impl FromVal for BaseAudioContext {
    fn from_val(v: &emlite::Val) -> Self {
        BaseAudioContext {
            inner: EventTarget::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for BaseAudioContext {
    type Target = EventTarget;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for BaseAudioContext {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<BaseAudioContext> for emlite::Val {
    fn from(s: BaseAudioContext) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl BaseAudioContext {
    pub fn destination(&self) -> AudioDestinationNode {
        self.inner.get("destination").as_::<AudioDestinationNode>()
    }
}
impl BaseAudioContext {
    pub fn sample_rate(&self) -> f32 {
        self.inner.get("sampleRate").as_::<f32>()
    }
}
impl BaseAudioContext {
    pub fn current_time(&self) -> f64 {
        self.inner.get("currentTime").as_::<f64>()
    }
}
impl BaseAudioContext {
    pub fn listener(&self) -> AudioListener {
        self.inner.get("listener").as_::<AudioListener>()
    }
}
impl BaseAudioContext {
    pub fn state(&self) -> AudioContextState {
        self.inner.get("state").as_::<AudioContextState>()
    }
}
impl BaseAudioContext {
    pub fn render_quantum_size(&self) -> u32 {
        self.inner.get("renderQuantumSize").as_::<u32>()
    }
}
impl BaseAudioContext {
    pub fn audio_worklet(&self) -> AudioWorklet {
        self.inner.get("audioWorklet").as_::<AudioWorklet>()
    }
}
impl BaseAudioContext {
    pub fn onstatechange(&self) -> jsbind::Any {
        self.inner.get("onstatechange").as_::<jsbind::Any>()
    }

    pub fn set_onstatechange(&mut self, value: jsbind::Any) {
        self.inner.set("onstatechange", value);
    }
}
impl BaseAudioContext {
    pub fn create_analyser(&self) -> AnalyserNode {
        self.inner.call("createAnalyser", &[]).as_::<AnalyserNode>()
    }
}
impl BaseAudioContext {
    pub fn create_biquad_filter(&self) -> BiquadFilterNode {
        self.inner
            .call("createBiquadFilter", &[])
            .as_::<BiquadFilterNode>()
    }
}
impl BaseAudioContext {
    pub fn create_buffer(
        &self,
        number_of_channels: u32,
        length: u32,
        sample_rate: f32,
    ) -> AudioBuffer {
        self.inner
            .call(
                "createBuffer",
                &[number_of_channels.into(), length.into(), sample_rate.into()],
            )
            .as_::<AudioBuffer>()
    }
}
impl BaseAudioContext {
    pub fn create_buffer_source(&self) -> AudioBufferSourceNode {
        self.inner
            .call("createBufferSource", &[])
            .as_::<AudioBufferSourceNode>()
    }
}
impl BaseAudioContext {
    pub fn create_channel_merger0(&self) -> ChannelMergerNode {
        self.inner
            .call("createChannelMerger", &[])
            .as_::<ChannelMergerNode>()
    }

    pub fn create_channel_merger1(&self, number_of_inputs: u32) -> ChannelMergerNode {
        self.inner
            .call("createChannelMerger", &[number_of_inputs.into()])
            .as_::<ChannelMergerNode>()
    }
}
impl BaseAudioContext {
    pub fn create_channel_splitter0(&self) -> ChannelSplitterNode {
        self.inner
            .call("createChannelSplitter", &[])
            .as_::<ChannelSplitterNode>()
    }

    pub fn create_channel_splitter1(&self, number_of_outputs: u32) -> ChannelSplitterNode {
        self.inner
            .call("createChannelSplitter", &[number_of_outputs.into()])
            .as_::<ChannelSplitterNode>()
    }
}
impl BaseAudioContext {
    pub fn create_constant_source(&self) -> ConstantSourceNode {
        self.inner
            .call("createConstantSource", &[])
            .as_::<ConstantSourceNode>()
    }
}
impl BaseAudioContext {
    pub fn create_convolver(&self) -> ConvolverNode {
        self.inner
            .call("createConvolver", &[])
            .as_::<ConvolverNode>()
    }
}
impl BaseAudioContext {
    pub fn create_delay0(&self) -> DelayNode {
        self.inner.call("createDelay", &[]).as_::<DelayNode>()
    }

    pub fn create_delay1(&self, max_delay_time: f64) -> DelayNode {
        self.inner
            .call("createDelay", &[max_delay_time.into()])
            .as_::<DelayNode>()
    }
}
impl BaseAudioContext {
    pub fn create_dynamics_compressor(&self) -> DynamicsCompressorNode {
        self.inner
            .call("createDynamicsCompressor", &[])
            .as_::<DynamicsCompressorNode>()
    }
}
impl BaseAudioContext {
    pub fn create_gain(&self) -> GainNode {
        self.inner.call("createGain", &[]).as_::<GainNode>()
    }
}
impl BaseAudioContext {
    pub fn create_iir_filter(
        &self,
        feedforward: jsbind::Sequence<f64>,
        feedback: jsbind::Sequence<f64>,
    ) -> IIRFilterNode {
        self.inner
            .call("createIIRFilter", &[feedforward.into(), feedback.into()])
            .as_::<IIRFilterNode>()
    }
}
impl BaseAudioContext {
    pub fn create_oscillator(&self) -> OscillatorNode {
        self.inner
            .call("createOscillator", &[])
            .as_::<OscillatorNode>()
    }
}
impl BaseAudioContext {
    pub fn create_panner(&self) -> PannerNode {
        self.inner.call("createPanner", &[]).as_::<PannerNode>()
    }
}
impl BaseAudioContext {
    pub fn create_periodic_wave0(
        &self,
        real: jsbind::Sequence<f32>,
        imag: jsbind::Sequence<f32>,
    ) -> PeriodicWave {
        self.inner
            .call("createPeriodicWave", &[real.into(), imag.into()])
            .as_::<PeriodicWave>()
    }

    pub fn create_periodic_wave1(
        &self,
        real: jsbind::Sequence<f32>,
        imag: jsbind::Sequence<f32>,
        constraints: PeriodicWaveConstraints,
    ) -> PeriodicWave {
        self.inner
            .call(
                "createPeriodicWave",
                &[real.into(), imag.into(), constraints.into()],
            )
            .as_::<PeriodicWave>()
    }
}
impl BaseAudioContext {
    pub fn create_script_processor0(&self) -> ScriptProcessorNode {
        self.inner
            .call("createScriptProcessor", &[])
            .as_::<ScriptProcessorNode>()
    }

    pub fn create_script_processor1(&self, buffer_size: u32) -> ScriptProcessorNode {
        self.inner
            .call("createScriptProcessor", &[buffer_size.into()])
            .as_::<ScriptProcessorNode>()
    }

    pub fn create_script_processor2(
        &self,
        buffer_size: u32,
        number_of_input_channels: u32,
    ) -> ScriptProcessorNode {
        self.inner
            .call(
                "createScriptProcessor",
                &[buffer_size.into(), number_of_input_channels.into()],
            )
            .as_::<ScriptProcessorNode>()
    }

    pub fn create_script_processor3(
        &self,
        buffer_size: u32,
        number_of_input_channels: u32,
        number_of_output_channels: u32,
    ) -> ScriptProcessorNode {
        self.inner
            .call(
                "createScriptProcessor",
                &[
                    buffer_size.into(),
                    number_of_input_channels.into(),
                    number_of_output_channels.into(),
                ],
            )
            .as_::<ScriptProcessorNode>()
    }
}
impl BaseAudioContext {
    pub fn create_stereo_panner(&self) -> StereoPannerNode {
        self.inner
            .call("createStereoPanner", &[])
            .as_::<StereoPannerNode>()
    }
}
impl BaseAudioContext {
    pub fn create_wave_shaper(&self) -> WaveShaperNode {
        self.inner
            .call("createWaveShaper", &[])
            .as_::<WaveShaperNode>()
    }
}
impl BaseAudioContext {
    pub fn decode_audio_data0(&self, audio_data: jsbind::ArrayBuffer) -> jsbind::Promise {
        self.inner
            .call("decodeAudioData", &[audio_data.into()])
            .as_::<jsbind::Promise>()
    }

    pub fn decode_audio_data1(
        &self,
        audio_data: jsbind::ArrayBuffer,
        success_callback: jsbind::Function,
    ) -> jsbind::Promise {
        self.inner
            .call(
                "decodeAudioData",
                &[audio_data.into(), success_callback.into()],
            )
            .as_::<jsbind::Promise>()
    }

    pub fn decode_audio_data2(
        &self,
        audio_data: jsbind::ArrayBuffer,
        success_callback: jsbind::Function,
        error_callback: jsbind::Function,
    ) -> jsbind::Promise {
        self.inner
            .call(
                "decodeAudioData",
                &[
                    audio_data.into(),
                    success_callback.into(),
                    error_callback.into(),
                ],
            )
            .as_::<jsbind::Promise>()
    }
}

use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct PeriodicWaveConstraints {
    inner: Any,
}
impl FromVal for PeriodicWaveConstraints {
    fn from_val(v: &Any) -> Self {
        PeriodicWaveConstraints { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for PeriodicWaveConstraints {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for PeriodicWaveConstraints {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for PeriodicWaveConstraints {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for PeriodicWaveConstraints {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<PeriodicWaveConstraints> for Any {
    fn from(s: PeriodicWaveConstraints) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&PeriodicWaveConstraints> for Any {
    fn from(s: &PeriodicWaveConstraints) -> Any {
        s.inner.clone()
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
/// The BaseAudioContext class.
/// [`BaseAudioContext`](https://developer.mozilla.org/en-US/docs/Web/API/BaseAudioContext)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct BaseAudioContext {
    inner: EventTarget,
}
impl FromVal for BaseAudioContext {
    fn from_val(v: &Any) -> Self {
        BaseAudioContext {
            inner: EventTarget::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
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
impl AsRef<Any> for BaseAudioContext {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for BaseAudioContext {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<BaseAudioContext> for Any {
    fn from(s: BaseAudioContext) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&BaseAudioContext> for Any {
    fn from(s: &BaseAudioContext) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(BaseAudioContext);

impl BaseAudioContext {
    /// Getter of the `destination` attribute.
    /// [`BaseAudioContext.destination`](https://developer.mozilla.org/en-US/docs/Web/API/BaseAudioContext/destination)
    pub fn destination(&self) -> AudioDestinationNode {
        self.inner.get("destination").as_::<AudioDestinationNode>()
    }
}
impl BaseAudioContext {
    /// Getter of the `sampleRate` attribute.
    /// [`BaseAudioContext.sampleRate`](https://developer.mozilla.org/en-US/docs/Web/API/BaseAudioContext/sampleRate)
    pub fn sample_rate(&self) -> f32 {
        self.inner.get("sampleRate").as_::<f32>()
    }
}
impl BaseAudioContext {
    /// Getter of the `currentTime` attribute.
    /// [`BaseAudioContext.currentTime`](https://developer.mozilla.org/en-US/docs/Web/API/BaseAudioContext/currentTime)
    pub fn current_time(&self) -> f64 {
        self.inner.get("currentTime").as_::<f64>()
    }
}
impl BaseAudioContext {
    /// Getter of the `listener` attribute.
    /// [`BaseAudioContext.listener`](https://developer.mozilla.org/en-US/docs/Web/API/BaseAudioContext/listener)
    pub fn listener(&self) -> AudioListener {
        self.inner.get("listener").as_::<AudioListener>()
    }
}
impl BaseAudioContext {
    /// Getter of the `state` attribute.
    /// [`BaseAudioContext.state`](https://developer.mozilla.org/en-US/docs/Web/API/BaseAudioContext/state)
    pub fn state(&self) -> AudioContextState {
        self.inner.get("state").as_::<AudioContextState>()
    }
}
impl BaseAudioContext {
    /// Getter of the `renderQuantumSize` attribute.
    /// [`BaseAudioContext.renderQuantumSize`](https://developer.mozilla.org/en-US/docs/Web/API/BaseAudioContext/renderQuantumSize)
    pub fn render_quantum_size(&self) -> u32 {
        self.inner.get("renderQuantumSize").as_::<u32>()
    }
}
impl BaseAudioContext {
    /// Getter of the `audioWorklet` attribute.
    /// [`BaseAudioContext.audioWorklet`](https://developer.mozilla.org/en-US/docs/Web/API/BaseAudioContext/audioWorklet)
    pub fn audio_worklet(&self) -> AudioWorklet {
        self.inner.get("audioWorklet").as_::<AudioWorklet>()
    }
}
impl BaseAudioContext {
    /// Getter of the `onstatechange` attribute.
    /// [`BaseAudioContext.onstatechange`](https://developer.mozilla.org/en-US/docs/Web/API/BaseAudioContext/onstatechange)
    pub fn onstatechange(&self) -> Any {
        self.inner.get("onstatechange").as_::<Any>()
    }

    /// Setter of the `onstatechange` attribute.
    /// [`BaseAudioContext.onstatechange`](https://developer.mozilla.org/en-US/docs/Web/API/BaseAudioContext/onstatechange)
    pub fn set_onstatechange(&mut self, value: &Any) {
        self.inner.set("onstatechange", value);
    }
}
impl BaseAudioContext {
    /// The createAnalyser method.
    /// [`BaseAudioContext.createAnalyser`](https://developer.mozilla.org/en-US/docs/Web/API/BaseAudioContext/createAnalyser)
    pub fn create_analyser(&self) -> AnalyserNode {
        self.inner.call("createAnalyser", &[]).as_::<AnalyserNode>()
    }
}
impl BaseAudioContext {
    /// The createBiquadFilter method.
    /// [`BaseAudioContext.createBiquadFilter`](https://developer.mozilla.org/en-US/docs/Web/API/BaseAudioContext/createBiquadFilter)
    pub fn create_biquad_filter(&self) -> BiquadFilterNode {
        self.inner
            .call("createBiquadFilter", &[])
            .as_::<BiquadFilterNode>()
    }
}
impl BaseAudioContext {
    /// The createBuffer method.
    /// [`BaseAudioContext.createBuffer`](https://developer.mozilla.org/en-US/docs/Web/API/BaseAudioContext/createBuffer)
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
    /// The createBufferSource method.
    /// [`BaseAudioContext.createBufferSource`](https://developer.mozilla.org/en-US/docs/Web/API/BaseAudioContext/createBufferSource)
    pub fn create_buffer_source(&self) -> AudioBufferSourceNode {
        self.inner
            .call("createBufferSource", &[])
            .as_::<AudioBufferSourceNode>()
    }
}
impl BaseAudioContext {
    /// The createChannelMerger method.
    /// [`BaseAudioContext.createChannelMerger`](https://developer.mozilla.org/en-US/docs/Web/API/BaseAudioContext/createChannelMerger)
    pub fn create_channel_merger0(&self) -> ChannelMergerNode {
        self.inner
            .call("createChannelMerger", &[])
            .as_::<ChannelMergerNode>()
    }
    /// The createChannelMerger method.
    /// [`BaseAudioContext.createChannelMerger`](https://developer.mozilla.org/en-US/docs/Web/API/BaseAudioContext/createChannelMerger)
    pub fn create_channel_merger1(&self, number_of_inputs: u32) -> ChannelMergerNode {
        self.inner
            .call("createChannelMerger", &[number_of_inputs.into()])
            .as_::<ChannelMergerNode>()
    }
}
impl BaseAudioContext {
    /// The createChannelSplitter method.
    /// [`BaseAudioContext.createChannelSplitter`](https://developer.mozilla.org/en-US/docs/Web/API/BaseAudioContext/createChannelSplitter)
    pub fn create_channel_splitter0(&self) -> ChannelSplitterNode {
        self.inner
            .call("createChannelSplitter", &[])
            .as_::<ChannelSplitterNode>()
    }
    /// The createChannelSplitter method.
    /// [`BaseAudioContext.createChannelSplitter`](https://developer.mozilla.org/en-US/docs/Web/API/BaseAudioContext/createChannelSplitter)
    pub fn create_channel_splitter1(&self, number_of_outputs: u32) -> ChannelSplitterNode {
        self.inner
            .call("createChannelSplitter", &[number_of_outputs.into()])
            .as_::<ChannelSplitterNode>()
    }
}
impl BaseAudioContext {
    /// The createConstantSource method.
    /// [`BaseAudioContext.createConstantSource`](https://developer.mozilla.org/en-US/docs/Web/API/BaseAudioContext/createConstantSource)
    pub fn create_constant_source(&self) -> ConstantSourceNode {
        self.inner
            .call("createConstantSource", &[])
            .as_::<ConstantSourceNode>()
    }
}
impl BaseAudioContext {
    /// The createConvolver method.
    /// [`BaseAudioContext.createConvolver`](https://developer.mozilla.org/en-US/docs/Web/API/BaseAudioContext/createConvolver)
    pub fn create_convolver(&self) -> ConvolverNode {
        self.inner
            .call("createConvolver", &[])
            .as_::<ConvolverNode>()
    }
}
impl BaseAudioContext {
    /// The createDelay method.
    /// [`BaseAudioContext.createDelay`](https://developer.mozilla.org/en-US/docs/Web/API/BaseAudioContext/createDelay)
    pub fn create_delay0(&self) -> DelayNode {
        self.inner.call("createDelay", &[]).as_::<DelayNode>()
    }
    /// The createDelay method.
    /// [`BaseAudioContext.createDelay`](https://developer.mozilla.org/en-US/docs/Web/API/BaseAudioContext/createDelay)
    pub fn create_delay1(&self, max_delay_time: f64) -> DelayNode {
        self.inner
            .call("createDelay", &[max_delay_time.into()])
            .as_::<DelayNode>()
    }
}
impl BaseAudioContext {
    /// The createDynamicsCompressor method.
    /// [`BaseAudioContext.createDynamicsCompressor`](https://developer.mozilla.org/en-US/docs/Web/API/BaseAudioContext/createDynamicsCompressor)
    pub fn create_dynamics_compressor(&self) -> DynamicsCompressorNode {
        self.inner
            .call("createDynamicsCompressor", &[])
            .as_::<DynamicsCompressorNode>()
    }
}
impl BaseAudioContext {
    /// The createGain method.
    /// [`BaseAudioContext.createGain`](https://developer.mozilla.org/en-US/docs/Web/API/BaseAudioContext/createGain)
    pub fn create_gain(&self) -> GainNode {
        self.inner.call("createGain", &[]).as_::<GainNode>()
    }
}
impl BaseAudioContext {
    /// The createIIRFilter method.
    /// [`BaseAudioContext.createIIRFilter`](https://developer.mozilla.org/en-US/docs/Web/API/BaseAudioContext/createIIRFilter)
    pub fn create_iir_filter(
        &self,
        feedforward: Sequence<f64>,
        feedback: Sequence<f64>,
    ) -> IIRFilterNode {
        self.inner
            .call("createIIRFilter", &[feedforward.into(), feedback.into()])
            .as_::<IIRFilterNode>()
    }
}
impl BaseAudioContext {
    /// The createOscillator method.
    /// [`BaseAudioContext.createOscillator`](https://developer.mozilla.org/en-US/docs/Web/API/BaseAudioContext/createOscillator)
    pub fn create_oscillator(&self) -> OscillatorNode {
        self.inner
            .call("createOscillator", &[])
            .as_::<OscillatorNode>()
    }
}
impl BaseAudioContext {
    /// The createPanner method.
    /// [`BaseAudioContext.createPanner`](https://developer.mozilla.org/en-US/docs/Web/API/BaseAudioContext/createPanner)
    pub fn create_panner(&self) -> PannerNode {
        self.inner.call("createPanner", &[]).as_::<PannerNode>()
    }
}
impl BaseAudioContext {
    /// The createPeriodicWave method.
    /// [`BaseAudioContext.createPeriodicWave`](https://developer.mozilla.org/en-US/docs/Web/API/BaseAudioContext/createPeriodicWave)
    pub fn create_periodic_wave0(&self, real: Sequence<f32>, imag: Sequence<f32>) -> PeriodicWave {
        self.inner
            .call("createPeriodicWave", &[real.into(), imag.into()])
            .as_::<PeriodicWave>()
    }
    /// The createPeriodicWave method.
    /// [`BaseAudioContext.createPeriodicWave`](https://developer.mozilla.org/en-US/docs/Web/API/BaseAudioContext/createPeriodicWave)
    pub fn create_periodic_wave1(
        &self,
        real: Sequence<f32>,
        imag: Sequence<f32>,
        constraints: &PeriodicWaveConstraints,
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
    /// The createScriptProcessor method.
    /// [`BaseAudioContext.createScriptProcessor`](https://developer.mozilla.org/en-US/docs/Web/API/BaseAudioContext/createScriptProcessor)
    pub fn create_script_processor0(&self) -> ScriptProcessorNode {
        self.inner
            .call("createScriptProcessor", &[])
            .as_::<ScriptProcessorNode>()
    }
    /// The createScriptProcessor method.
    /// [`BaseAudioContext.createScriptProcessor`](https://developer.mozilla.org/en-US/docs/Web/API/BaseAudioContext/createScriptProcessor)
    pub fn create_script_processor1(&self, buffer_size: u32) -> ScriptProcessorNode {
        self.inner
            .call("createScriptProcessor", &[buffer_size.into()])
            .as_::<ScriptProcessorNode>()
    }
    /// The createScriptProcessor method.
    /// [`BaseAudioContext.createScriptProcessor`](https://developer.mozilla.org/en-US/docs/Web/API/BaseAudioContext/createScriptProcessor)
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
    /// The createScriptProcessor method.
    /// [`BaseAudioContext.createScriptProcessor`](https://developer.mozilla.org/en-US/docs/Web/API/BaseAudioContext/createScriptProcessor)
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
    /// The createStereoPanner method.
    /// [`BaseAudioContext.createStereoPanner`](https://developer.mozilla.org/en-US/docs/Web/API/BaseAudioContext/createStereoPanner)
    pub fn create_stereo_panner(&self) -> StereoPannerNode {
        self.inner
            .call("createStereoPanner", &[])
            .as_::<StereoPannerNode>()
    }
}
impl BaseAudioContext {
    /// The createWaveShaper method.
    /// [`BaseAudioContext.createWaveShaper`](https://developer.mozilla.org/en-US/docs/Web/API/BaseAudioContext/createWaveShaper)
    pub fn create_wave_shaper(&self) -> WaveShaperNode {
        self.inner
            .call("createWaveShaper", &[])
            .as_::<WaveShaperNode>()
    }
}
impl BaseAudioContext {
    /// The decodeAudioData method.
    /// [`BaseAudioContext.decodeAudioData`](https://developer.mozilla.org/en-US/docs/Web/API/BaseAudioContext/decodeAudioData)
    pub fn decode_audio_data0(&self, audio_data: &ArrayBuffer) -> Promise<AudioBuffer> {
        self.inner
            .call("decodeAudioData", &[audio_data.into()])
            .as_::<Promise<AudioBuffer>>()
    }
    /// The decodeAudioData method.
    /// [`BaseAudioContext.decodeAudioData`](https://developer.mozilla.org/en-US/docs/Web/API/BaseAudioContext/decodeAudioData)
    pub fn decode_audio_data1(
        &self,
        audio_data: &ArrayBuffer,
        success_callback: &Function,
    ) -> Promise<AudioBuffer> {
        self.inner
            .call(
                "decodeAudioData",
                &[audio_data.into(), success_callback.into()],
            )
            .as_::<Promise<AudioBuffer>>()
    }
    /// The decodeAudioData method.
    /// [`BaseAudioContext.decodeAudioData`](https://developer.mozilla.org/en-US/docs/Web/API/BaseAudioContext/decodeAudioData)
    pub fn decode_audio_data2(
        &self,
        audio_data: &ArrayBuffer,
        success_callback: &Function,
        error_callback: &Function,
    ) -> Promise<AudioBuffer> {
        self.inner
            .call(
                "decodeAudioData",
                &[
                    audio_data.into(),
                    success_callback.into(),
                    error_callback.into(),
                ],
            )
            .as_::<Promise<AudioBuffer>>()
    }
}

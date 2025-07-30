use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct AudioDecoderConfig {
    inner: Any,
}
impl FromVal for AudioDecoderConfig {
    fn from_val(v: &Any) -> Self {
        AudioDecoderConfig { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for AudioDecoderConfig {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for AudioDecoderConfig {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for AudioDecoderConfig {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for AudioDecoderConfig {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<AudioDecoderConfig> for Any {
    fn from(s: AudioDecoderConfig) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&AudioDecoderConfig> for Any {
    fn from(s: &AudioDecoderConfig) -> Any {
        s.inner.clone()
    }
}

impl AudioDecoderConfig {
    pub fn codec(&self) -> JsString {
        self.inner.get("codec").as_::<JsString>()
    }

    pub fn set_codec(&mut self, value: &JsString) {
        self.inner.set("codec", value);
    }
}
impl AudioDecoderConfig {
    pub fn sample_rate(&self) -> u32 {
        self.inner.get("sampleRate").as_::<u32>()
    }

    pub fn set_sample_rate(&mut self, value: u32) {
        self.inner.set("sampleRate", value);
    }
}
impl AudioDecoderConfig {
    pub fn number_of_channels(&self) -> u32 {
        self.inner.get("numberOfChannels").as_::<u32>()
    }

    pub fn set_number_of_channels(&mut self, value: u32) {
        self.inner.set("numberOfChannels", value);
    }
}
impl AudioDecoderConfig {
    pub fn description(&self) -> Any {
        self.inner.get("description").as_::<Any>()
    }

    pub fn set_description(&mut self, value: &Any) {
        self.inner.set("description", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct AudioDecoderSupport {
    inner: Any,
}
impl FromVal for AudioDecoderSupport {
    fn from_val(v: &Any) -> Self {
        AudioDecoderSupport { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for AudioDecoderSupport {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for AudioDecoderSupport {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for AudioDecoderSupport {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for AudioDecoderSupport {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<AudioDecoderSupport> for Any {
    fn from(s: AudioDecoderSupport) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&AudioDecoderSupport> for Any {
    fn from(s: &AudioDecoderSupport) -> Any {
        s.inner.clone()
    }
}

impl AudioDecoderSupport {
    pub fn supported(&self) -> bool {
        self.inner.get("supported").as_::<bool>()
    }

    pub fn set_supported(&mut self, value: bool) {
        self.inner.set("supported", value);
    }
}
impl AudioDecoderSupport {
    pub fn config(&self) -> AudioDecoderConfig {
        self.inner.get("config").as_::<AudioDecoderConfig>()
    }

    pub fn set_config(&mut self, value: &AudioDecoderConfig) {
        self.inner.set("config", value);
    }
}
/// The AudioDecoder class.
/// [`AudioDecoder`](https://developer.mozilla.org/en-US/docs/Web/API/AudioDecoder)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct AudioDecoder {
    inner: EventTarget,
}
impl FromVal for AudioDecoder {
    fn from_val(v: &Any) -> Self {
        AudioDecoder {
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
impl core::ops::Deref for AudioDecoder {
    type Target = EventTarget;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for AudioDecoder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for AudioDecoder {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for AudioDecoder {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<AudioDecoder> for Any {
    fn from(s: AudioDecoder) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&AudioDecoder> for Any {
    fn from(s: &AudioDecoder) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(AudioDecoder);

impl AudioDecoder {
    /// The `new AudioDecoder(..)` constructor, creating a new AudioDecoder instance
    pub fn new(init: &Any) -> AudioDecoder {
        Self {
            inner: Any::global("AudioDecoder")
                .new(&[init.into()])
                .as_::<EventTarget>(),
        }
    }
}
impl AudioDecoder {
    /// Getter of the `state` attribute.
    /// [`AudioDecoder.state`](https://developer.mozilla.org/en-US/docs/Web/API/AudioDecoder/state)
    pub fn state(&self) -> CodecState {
        self.inner.get("state").as_::<CodecState>()
    }
}
impl AudioDecoder {
    /// Getter of the `decodeQueueSize` attribute.
    /// [`AudioDecoder.decodeQueueSize`](https://developer.mozilla.org/en-US/docs/Web/API/AudioDecoder/decodeQueueSize)
    pub fn decode_queue_size(&self) -> u32 {
        self.inner.get("decodeQueueSize").as_::<u32>()
    }
}
impl AudioDecoder {
    /// Getter of the `ondequeue` attribute.
    /// [`AudioDecoder.ondequeue`](https://developer.mozilla.org/en-US/docs/Web/API/AudioDecoder/ondequeue)
    pub fn ondequeue(&self) -> Any {
        self.inner.get("ondequeue").as_::<Any>()
    }

    /// Setter of the `ondequeue` attribute.
    /// [`AudioDecoder.ondequeue`](https://developer.mozilla.org/en-US/docs/Web/API/AudioDecoder/ondequeue)
    pub fn set_ondequeue(&mut self, value: &Any) {
        self.inner.set("ondequeue", value);
    }
}
impl AudioDecoder {
    /// The configure method.
    /// [`AudioDecoder.configure`](https://developer.mozilla.org/en-US/docs/Web/API/AudioDecoder/configure)
    pub fn configure(&self, config: &AudioDecoderConfig) -> Undefined {
        self.inner
            .call("configure", &[config.into()])
            .as_::<Undefined>()
    }
}
impl AudioDecoder {
    /// The decode method.
    /// [`AudioDecoder.decode`](https://developer.mozilla.org/en-US/docs/Web/API/AudioDecoder/decode)
    pub fn decode(&self, chunk: &EncodedAudioChunk) -> Undefined {
        self.inner
            .call("decode", &[chunk.into()])
            .as_::<Undefined>()
    }
}
impl AudioDecoder {
    /// The flush method.
    /// [`AudioDecoder.flush`](https://developer.mozilla.org/en-US/docs/Web/API/AudioDecoder/flush)
    pub fn flush(&self) -> Promise<Undefined> {
        self.inner.call("flush", &[]).as_::<Promise<Undefined>>()
    }
}
impl AudioDecoder {
    /// The reset method.
    /// [`AudioDecoder.reset`](https://developer.mozilla.org/en-US/docs/Web/API/AudioDecoder/reset)
    pub fn reset(&self) -> Undefined {
        self.inner.call("reset", &[]).as_::<Undefined>()
    }
}
impl AudioDecoder {
    /// The close method.
    /// [`AudioDecoder.close`](https://developer.mozilla.org/en-US/docs/Web/API/AudioDecoder/close)
    pub fn close(&self) -> Undefined {
        self.inner.call("close", &[]).as_::<Undefined>()
    }
}
impl AudioDecoder {
    /// The isConfigSupported method.
    /// [`AudioDecoder.isConfigSupported`](https://developer.mozilla.org/en-US/docs/Web/API/AudioDecoder/isConfigSupported)
    pub fn is_config_supported(config: &AudioDecoderConfig) -> Promise<AudioDecoderSupport> {
        Any::global("AudioDecoder")
            .call("isConfigSupported", &[config.into()])
            .as_::<Promise<AudioDecoderSupport>>()
    }
}

use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct AudioDecoderConfig {
    inner: emlite::Val,
}
impl FromVal for AudioDecoderConfig {
    fn from_val(v: &emlite::Val) -> Self {
        AudioDecoderConfig { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for AudioDecoderConfig {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for AudioDecoderConfig {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for AudioDecoderConfig {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for AudioDecoderConfig {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<AudioDecoderConfig> for emlite::Val {
    fn from(s: AudioDecoderConfig) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl AudioDecoderConfig {
    pub fn codec(&self) -> DOMString {
        self.inner.get("codec").as_::<DOMString>()
    }

    pub fn set_codec(&mut self, value: DOMString) {
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

    pub fn set_description(&mut self, value: Any) {
        self.inner.set("description", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct AudioDecoderSupport {
    inner: emlite::Val,
}
impl FromVal for AudioDecoderSupport {
    fn from_val(v: &emlite::Val) -> Self {
        AudioDecoderSupport { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for AudioDecoderSupport {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for AudioDecoderSupport {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for AudioDecoderSupport {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for AudioDecoderSupport {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<AudioDecoderSupport> for emlite::Val {
    fn from(s: AudioDecoderSupport) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
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

    pub fn set_config(&mut self, value: AudioDecoderConfig) {
        self.inner.set("config", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct AudioDecoder {
    inner: EventTarget,
}
impl FromVal for AudioDecoder {
    fn from_val(v: &emlite::Val) -> Self {
        AudioDecoder {
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
impl AsRef<emlite::Val> for AudioDecoder {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for AudioDecoder {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<AudioDecoder> for emlite::Val {
    fn from(s: AudioDecoder) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(AudioDecoder);

impl AudioDecoder {
    pub fn new(init: Any) -> AudioDecoder {
        Self {
            inner: emlite::Val::global("AudioDecoder")
                .new(&[init.into()])
                .as_::<EventTarget>(),
        }
    }
}
impl AudioDecoder {
    pub fn state(&self) -> CodecState {
        self.inner.get("state").as_::<CodecState>()
    }
}
impl AudioDecoder {
    pub fn decode_queue_size(&self) -> u32 {
        self.inner.get("decodeQueueSize").as_::<u32>()
    }
}
impl AudioDecoder {
    pub fn ondequeue(&self) -> Any {
        self.inner.get("ondequeue").as_::<Any>()
    }

    pub fn set_ondequeue(&mut self, value: Any) {
        self.inner.set("ondequeue", value);
    }
}
impl AudioDecoder {
    pub fn configure(&self, config: AudioDecoderConfig) -> Undefined {
        self.inner
            .call("configure", &[config.into()])
            .as_::<Undefined>()
    }
}
impl AudioDecoder {
    pub fn decode(&self, chunk: EncodedAudioChunk) -> Undefined {
        self.inner
            .call("decode", &[chunk.into()])
            .as_::<Undefined>()
    }
}
impl AudioDecoder {
    pub fn flush(&self) -> Promise {
        self.inner.call("flush", &[]).as_::<Promise>()
    }
}
impl AudioDecoder {
    pub fn reset(&self) -> Undefined {
        self.inner.call("reset", &[]).as_::<Undefined>()
    }
}
impl AudioDecoder {
    pub fn close(&self) -> Undefined {
        self.inner.call("close", &[]).as_::<Undefined>()
    }
}
impl AudioDecoder {
    pub fn is_config_supported(config: AudioDecoderConfig) -> Promise {
        emlite::Val::global("AudioDecoder")
            .call("isConfigSupported", &[config.into()])
            .as_::<Promise>()
    }
}

use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct AudioEncoderConfig {
    inner: emlite::Val,
}
impl FromVal for AudioEncoderConfig {
    fn from_val(v: &emlite::Val) -> Self {
        AudioEncoderConfig { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for AudioEncoderConfig {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for AudioEncoderConfig {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for AudioEncoderConfig {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for AudioEncoderConfig {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<AudioEncoderConfig> for emlite::Val {
    fn from(s: AudioEncoderConfig) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&AudioEncoderConfig> for emlite::Val {
    fn from(s: &AudioEncoderConfig) -> emlite::Val {
        s.inner.clone()
    }
}

impl AudioEncoderConfig {
    pub fn codec(&self) -> DOMString {
        self.inner.get("codec").as_::<DOMString>()
    }

    pub fn set_codec(&mut self, value: DOMString) {
        self.inner.set("codec", value);
    }
}
impl AudioEncoderConfig {
    pub fn sample_rate(&self) -> u32 {
        self.inner.get("sampleRate").as_::<u32>()
    }

    pub fn set_sample_rate(&mut self, value: u32) {
        self.inner.set("sampleRate", value);
    }
}
impl AudioEncoderConfig {
    pub fn number_of_channels(&self) -> u32 {
        self.inner.get("numberOfChannels").as_::<u32>()
    }

    pub fn set_number_of_channels(&mut self, value: u32) {
        self.inner.set("numberOfChannels", value);
    }
}
impl AudioEncoderConfig {
    pub fn bitrate(&self) -> u64 {
        self.inner.get("bitrate").as_::<u64>()
    }

    pub fn set_bitrate(&mut self, value: u64) {
        self.inner.set("bitrate", value);
    }
}
impl AudioEncoderConfig {
    pub fn bitrate_mode(&self) -> BitrateMode {
        self.inner.get("bitrateMode").as_::<BitrateMode>()
    }

    pub fn set_bitrate_mode(&mut self, value: BitrateMode) {
        self.inner.set("bitrateMode", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct AudioEncoderSupport {
    inner: emlite::Val,
}
impl FromVal for AudioEncoderSupport {
    fn from_val(v: &emlite::Val) -> Self {
        AudioEncoderSupport { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for AudioEncoderSupport {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for AudioEncoderSupport {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for AudioEncoderSupport {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for AudioEncoderSupport {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<AudioEncoderSupport> for emlite::Val {
    fn from(s: AudioEncoderSupport) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&AudioEncoderSupport> for emlite::Val {
    fn from(s: &AudioEncoderSupport) -> emlite::Val {
        s.inner.clone()
    }
}

impl AudioEncoderSupport {
    pub fn supported(&self) -> bool {
        self.inner.get("supported").as_::<bool>()
    }

    pub fn set_supported(&mut self, value: bool) {
        self.inner.set("supported", value);
    }
}
impl AudioEncoderSupport {
    pub fn config(&self) -> AudioEncoderConfig {
        self.inner.get("config").as_::<AudioEncoderConfig>()
    }

    pub fn set_config(&mut self, value: AudioEncoderConfig) {
        self.inner.set("config", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct AudioEncoder {
    inner: EventTarget,
}
impl FromVal for AudioEncoder {
    fn from_val(v: &emlite::Val) -> Self {
        AudioEncoder {
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
impl core::ops::Deref for AudioEncoder {
    type Target = EventTarget;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for AudioEncoder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for AudioEncoder {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for AudioEncoder {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<AudioEncoder> for emlite::Val {
    fn from(s: AudioEncoder) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&AudioEncoder> for emlite::Val {
    fn from(s: &AudioEncoder) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(AudioEncoder);

impl AudioEncoder {
    pub fn new(init: Any) -> AudioEncoder {
        Self {
            inner: emlite::Val::global("AudioEncoder")
                .new(&[init.into()])
                .as_::<EventTarget>(),
        }
    }
}
impl AudioEncoder {
    pub fn state(&self) -> CodecState {
        self.inner.get("state").as_::<CodecState>()
    }
}
impl AudioEncoder {
    pub fn encode_queue_size(&self) -> u32 {
        self.inner.get("encodeQueueSize").as_::<u32>()
    }
}
impl AudioEncoder {
    pub fn ondequeue(&self) -> Any {
        self.inner.get("ondequeue").as_::<Any>()
    }

    pub fn set_ondequeue(&mut self, value: Any) {
        self.inner.set("ondequeue", value);
    }
}
impl AudioEncoder {
    pub fn configure(&self, config: AudioEncoderConfig) -> Undefined {
        self.inner
            .call("configure", &[config.into()])
            .as_::<Undefined>()
    }
}
impl AudioEncoder {
    pub fn encode(&self, data: AudioData) -> Undefined {
        self.inner.call("encode", &[data.into()]).as_::<Undefined>()
    }
}
impl AudioEncoder {
    pub fn flush(&self) -> Promise {
        self.inner.call("flush", &[]).as_::<Promise>()
    }
}
impl AudioEncoder {
    pub fn reset(&self) -> Undefined {
        self.inner.call("reset", &[]).as_::<Undefined>()
    }
}
impl AudioEncoder {
    pub fn close(&self) -> Undefined {
        self.inner.call("close", &[]).as_::<Undefined>()
    }
}
impl AudioEncoder {
    pub fn is_config_supported(config: AudioEncoderConfig) -> Promise {
        emlite::Val::global("AudioEncoder")
            .call("isConfigSupported", &[config.into()])
            .as_::<Promise>()
    }
}

use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct VideoEncoderConfig {
    inner: emlite::Val,
}
impl FromVal for VideoEncoderConfig {
    fn from_val(v: &emlite::Val) -> Self {
        VideoEncoderConfig { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for VideoEncoderConfig {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for VideoEncoderConfig {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for VideoEncoderConfig {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for VideoEncoderConfig {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<VideoEncoderConfig> for emlite::Val {
    fn from(s: VideoEncoderConfig) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl VideoEncoderConfig {
    pub fn codec(&self) -> jsbind::DOMString {
        self.inner.get("codec").as_::<jsbind::DOMString>()
    }

    pub fn set_codec(&mut self, value: jsbind::DOMString) {
        self.inner.set("codec", value);
    }
}
impl VideoEncoderConfig {
    pub fn width(&self) -> u32 {
        self.inner.get("width").as_::<u32>()
    }

    pub fn set_width(&mut self, value: u32) {
        self.inner.set("width", value);
    }
}
impl VideoEncoderConfig {
    pub fn height(&self) -> u32 {
        self.inner.get("height").as_::<u32>()
    }

    pub fn set_height(&mut self, value: u32) {
        self.inner.set("height", value);
    }
}
impl VideoEncoderConfig {
    pub fn display_width(&self) -> u32 {
        self.inner.get("displayWidth").as_::<u32>()
    }

    pub fn set_display_width(&mut self, value: u32) {
        self.inner.set("displayWidth", value);
    }
}
impl VideoEncoderConfig {
    pub fn display_height(&self) -> u32 {
        self.inner.get("displayHeight").as_::<u32>()
    }

    pub fn set_display_height(&mut self, value: u32) {
        self.inner.set("displayHeight", value);
    }
}
impl VideoEncoderConfig {
    pub fn bitrate(&self) -> u64 {
        self.inner.get("bitrate").as_::<u64>()
    }

    pub fn set_bitrate(&mut self, value: u64) {
        self.inner.set("bitrate", value);
    }
}
impl VideoEncoderConfig {
    pub fn framerate(&self) -> f64 {
        self.inner.get("framerate").as_::<f64>()
    }

    pub fn set_framerate(&mut self, value: f64) {
        self.inner.set("framerate", value);
    }
}
impl VideoEncoderConfig {
    pub fn hardware_acceleration(&self) -> HardwareAcceleration {
        self.inner
            .get("hardwareAcceleration")
            .as_::<HardwareAcceleration>()
    }

    pub fn set_hardware_acceleration(&mut self, value: HardwareAcceleration) {
        self.inner.set("hardwareAcceleration", value);
    }
}
impl VideoEncoderConfig {
    pub fn alpha(&self) -> AlphaOption {
        self.inner.get("alpha").as_::<AlphaOption>()
    }

    pub fn set_alpha(&mut self, value: AlphaOption) {
        self.inner.set("alpha", value);
    }
}
impl VideoEncoderConfig {
    pub fn scalability_mode(&self) -> jsbind::DOMString {
        self.inner.get("scalabilityMode").as_::<jsbind::DOMString>()
    }

    pub fn set_scalability_mode(&mut self, value: jsbind::DOMString) {
        self.inner.set("scalabilityMode", value);
    }
}
impl VideoEncoderConfig {
    pub fn bitrate_mode(&self) -> VideoEncoderBitrateMode {
        self.inner
            .get("bitrateMode")
            .as_::<VideoEncoderBitrateMode>()
    }

    pub fn set_bitrate_mode(&mut self, value: VideoEncoderBitrateMode) {
        self.inner.set("bitrateMode", value);
    }
}
impl VideoEncoderConfig {
    pub fn latency_mode(&self) -> LatencyMode {
        self.inner.get("latencyMode").as_::<LatencyMode>()
    }

    pub fn set_latency_mode(&mut self, value: LatencyMode) {
        self.inner.set("latencyMode", value);
    }
}
impl VideoEncoderConfig {
    pub fn content_hint(&self) -> jsbind::DOMString {
        self.inner.get("contentHint").as_::<jsbind::DOMString>()
    }

    pub fn set_content_hint(&mut self, value: jsbind::DOMString) {
        self.inner.set("contentHint", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct VideoEncoderEncodeOptions {
    inner: emlite::Val,
}
impl FromVal for VideoEncoderEncodeOptions {
    fn from_val(v: &emlite::Val) -> Self {
        VideoEncoderEncodeOptions { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for VideoEncoderEncodeOptions {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for VideoEncoderEncodeOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for VideoEncoderEncodeOptions {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for VideoEncoderEncodeOptions {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<VideoEncoderEncodeOptions> for emlite::Val {
    fn from(s: VideoEncoderEncodeOptions) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl VideoEncoderEncodeOptions {
    pub fn key_frame(&self) -> bool {
        self.inner.get("keyFrame").as_::<bool>()
    }

    pub fn set_key_frame(&mut self, value: bool) {
        self.inner.set("keyFrame", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct VideoEncoderSupport {
    inner: emlite::Val,
}
impl FromVal for VideoEncoderSupport {
    fn from_val(v: &emlite::Val) -> Self {
        VideoEncoderSupport { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for VideoEncoderSupport {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for VideoEncoderSupport {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for VideoEncoderSupport {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for VideoEncoderSupport {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<VideoEncoderSupport> for emlite::Val {
    fn from(s: VideoEncoderSupport) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl VideoEncoderSupport {
    pub fn supported(&self) -> bool {
        self.inner.get("supported").as_::<bool>()
    }

    pub fn set_supported(&mut self, value: bool) {
        self.inner.set("supported", value);
    }
}
impl VideoEncoderSupport {
    pub fn config(&self) -> VideoEncoderConfig {
        self.inner.get("config").as_::<VideoEncoderConfig>()
    }

    pub fn set_config(&mut self, value: VideoEncoderConfig) {
        self.inner.set("config", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct VideoEncoder {
    inner: EventTarget,
}
impl FromVal for VideoEncoder {
    fn from_val(v: &emlite::Val) -> Self {
        VideoEncoder {
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
impl core::ops::Deref for VideoEncoder {
    type Target = EventTarget;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for VideoEncoder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for VideoEncoder {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for VideoEncoder {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<VideoEncoder> for emlite::Val {
    fn from(s: VideoEncoder) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(VideoEncoder);

impl VideoEncoder {
    pub fn new(init: jsbind::Any) -> VideoEncoder {
        Self {
            inner: emlite::Val::global("VideoEncoder")
                .new(&[init.into()])
                .as_::<EventTarget>(),
        }
    }
}
impl VideoEncoder {
    pub fn state(&self) -> CodecState {
        self.inner.get("state").as_::<CodecState>()
    }
}
impl VideoEncoder {
    pub fn encode_queue_size(&self) -> u32 {
        self.inner.get("encodeQueueSize").as_::<u32>()
    }
}
impl VideoEncoder {
    pub fn ondequeue(&self) -> jsbind::Any {
        self.inner.get("ondequeue").as_::<jsbind::Any>()
    }

    pub fn set_ondequeue(&mut self, value: jsbind::Any) {
        self.inner.set("ondequeue", value);
    }
}
impl VideoEncoder {
    pub fn configure(&self, config: VideoEncoderConfig) -> jsbind::Undefined {
        self.inner
            .call("configure", &[config.into()])
            .as_::<jsbind::Undefined>()
    }
}
impl VideoEncoder {
    pub fn encode0(&self, frame: VideoFrame) -> jsbind::Undefined {
        self.inner
            .call("encode", &[frame.into()])
            .as_::<jsbind::Undefined>()
    }

    pub fn encode1(
        &self,
        frame: VideoFrame,
        options: VideoEncoderEncodeOptions,
    ) -> jsbind::Undefined {
        self.inner
            .call("encode", &[frame.into(), options.into()])
            .as_::<jsbind::Undefined>()
    }
}
impl VideoEncoder {
    pub fn flush(&self) -> jsbind::Promise {
        self.inner.call("flush", &[]).as_::<jsbind::Promise>()
    }
}
impl VideoEncoder {
    pub fn reset(&self) -> jsbind::Undefined {
        self.inner.call("reset", &[]).as_::<jsbind::Undefined>()
    }
}
impl VideoEncoder {
    pub fn close(&self) -> jsbind::Undefined {
        self.inner.call("close", &[]).as_::<jsbind::Undefined>()
    }
}
impl VideoEncoder {
    pub fn is_config_supported(config: VideoEncoderConfig) -> jsbind::Promise {
        emlite::Val::global("videoencoder")
            .call("isConfigSupported", &[config.into()])
            .as_::<jsbind::Promise>()
    }
}

use super::*;

#[derive(Clone, Debug)]
pub struct VideoDecoderConfig {
    inner: emlite::Val,
}
impl FromVal for VideoDecoderConfig {
    fn from_val(v: &emlite::Val) -> Self {
        VideoDecoderConfig { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for VideoDecoderConfig {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for VideoDecoderConfig {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<VideoDecoderConfig> for emlite::Val {
    fn from(s: VideoDecoderConfig) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl VideoDecoderConfig {
    pub fn codec(&self) -> jsbind::DOMString {
        self.inner.get("codec").as_::<jsbind::DOMString>()
    }

    pub fn set_codec(&mut self, value: jsbind::DOMString) {
        self.inner.set("codec", value);
    }
}
impl VideoDecoderConfig {
    pub fn description(&self) -> jsbind::Any {
        self.inner.get("description").as_::<jsbind::Any>()
    }

    pub fn set_description(&mut self, value: jsbind::Any) {
        self.inner.set("description", value);
    }
}
impl VideoDecoderConfig {
    pub fn coded_width(&self) -> u32 {
        self.inner.get("codedWidth").as_::<u32>()
    }

    pub fn set_coded_width(&mut self, value: u32) {
        self.inner.set("codedWidth", value);
    }
}
impl VideoDecoderConfig {
    pub fn coded_height(&self) -> u32 {
        self.inner.get("codedHeight").as_::<u32>()
    }

    pub fn set_coded_height(&mut self, value: u32) {
        self.inner.set("codedHeight", value);
    }
}
impl VideoDecoderConfig {
    pub fn display_aspect_width(&self) -> u32 {
        self.inner.get("displayAspectWidth").as_::<u32>()
    }

    pub fn set_display_aspect_width(&mut self, value: u32) {
        self.inner.set("displayAspectWidth", value);
    }
}
impl VideoDecoderConfig {
    pub fn display_aspect_height(&self) -> u32 {
        self.inner.get("displayAspectHeight").as_::<u32>()
    }

    pub fn set_display_aspect_height(&mut self, value: u32) {
        self.inner.set("displayAspectHeight", value);
    }
}
impl VideoDecoderConfig {
    pub fn color_space(&self) -> VideoColorSpaceInit {
        self.inner.get("colorSpace").as_::<VideoColorSpaceInit>()
    }

    pub fn set_color_space(&mut self, value: VideoColorSpaceInit) {
        self.inner.set("colorSpace", value);
    }
}
impl VideoDecoderConfig {
    pub fn hardware_acceleration(&self) -> HardwareAcceleration {
        self.inner
            .get("hardwareAcceleration")
            .as_::<HardwareAcceleration>()
    }

    pub fn set_hardware_acceleration(&mut self, value: HardwareAcceleration) {
        self.inner.set("hardwareAcceleration", value);
    }
}
impl VideoDecoderConfig {
    pub fn optimize_for_latency(&self) -> bool {
        self.inner.get("optimizeForLatency").as_::<bool>()
    }

    pub fn set_optimize_for_latency(&mut self, value: bool) {
        self.inner.set("optimizeForLatency", value);
    }
}
impl VideoDecoderConfig {
    pub fn rotation(&self) -> f64 {
        self.inner.get("rotation").as_::<f64>()
    }

    pub fn set_rotation(&mut self, value: f64) {
        self.inner.set("rotation", value);
    }
}
impl VideoDecoderConfig {
    pub fn flip(&self) -> bool {
        self.inner.get("flip").as_::<bool>()
    }

    pub fn set_flip(&mut self, value: bool) {
        self.inner.set("flip", value);
    }
}
#[derive(Clone, Debug)]
pub struct VideoDecoderSupport {
    inner: emlite::Val,
}
impl FromVal for VideoDecoderSupport {
    fn from_val(v: &emlite::Val) -> Self {
        VideoDecoderSupport { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for VideoDecoderSupport {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for VideoDecoderSupport {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<VideoDecoderSupport> for emlite::Val {
    fn from(s: VideoDecoderSupport) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl VideoDecoderSupport {
    pub fn supported(&self) -> bool {
        self.inner.get("supported").as_::<bool>()
    }

    pub fn set_supported(&mut self, value: bool) {
        self.inner.set("supported", value);
    }
}
impl VideoDecoderSupport {
    pub fn config(&self) -> VideoDecoderConfig {
        self.inner.get("config").as_::<VideoDecoderConfig>()
    }

    pub fn set_config(&mut self, value: VideoDecoderConfig) {
        self.inner.set("config", value);
    }
}
#[derive(Clone, Debug)]
pub struct VideoDecoder {
    inner: EventTarget,
}
impl FromVal for VideoDecoder {
    fn from_val(v: &emlite::Val) -> Self {
        VideoDecoder {
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
impl std::ops::Deref for VideoDecoder {
    type Target = EventTarget;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for VideoDecoder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<VideoDecoder> for emlite::Val {
    fn from(s: VideoDecoder) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl VideoDecoder {
    pub fn new(init: jsbind::Any) -> VideoDecoder {
        Self {
            inner: emlite::Val::global("VideoDecoder")
                .new(&[init.into()])
                .as_::<EventTarget>(),
        }
    }
}
impl VideoDecoder {
    pub fn state(&self) -> CodecState {
        self.inner.get("state").as_::<CodecState>()
    }
}
impl VideoDecoder {
    pub fn decode_queue_size(&self) -> u32 {
        self.inner.get("decodeQueueSize").as_::<u32>()
    }
}
impl VideoDecoder {
    pub fn ondequeue(&self) -> jsbind::Any {
        self.inner.get("ondequeue").as_::<jsbind::Any>()
    }

    pub fn set_ondequeue(&mut self, value: jsbind::Any) {
        self.inner.set("ondequeue", value);
    }
}
impl VideoDecoder {
    pub fn configure(&self, config: VideoDecoderConfig) -> jsbind::Undefined {
        self.inner
            .call("configure", &[config.into()])
            .as_::<jsbind::Undefined>()
    }
}
impl VideoDecoder {
    pub fn decode(&self, chunk: EncodedVideoChunk) -> jsbind::Undefined {
        self.inner
            .call("decode", &[chunk.into()])
            .as_::<jsbind::Undefined>()
    }
}
impl VideoDecoder {
    pub fn flush(&self) -> jsbind::Promise {
        self.inner.call("flush", &[]).as_::<jsbind::Promise>()
    }
}
impl VideoDecoder {
    pub fn reset(&self) -> jsbind::Undefined {
        self.inner.call("reset", &[]).as_::<jsbind::Undefined>()
    }
}
impl VideoDecoder {
    pub fn close(&self) -> jsbind::Undefined {
        self.inner.call("close", &[]).as_::<jsbind::Undefined>()
    }
}
impl VideoDecoder {
    pub fn is_config_supported(config: VideoDecoderConfig) -> jsbind::Promise {
        emlite::Val::global("videodecoder")
            .call("isConfigSupported", &[config.into()])
            .as_::<jsbind::Promise>()
    }
}

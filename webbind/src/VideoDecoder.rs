use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct VideoDecoderConfig {
    inner: Any,
}
impl FromVal for VideoDecoderConfig {
    fn from_val(v: &Any) -> Self {
        VideoDecoderConfig { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for VideoDecoderConfig {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for VideoDecoderConfig {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for VideoDecoderConfig {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for VideoDecoderConfig {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<VideoDecoderConfig> for Any {
    fn from(s: VideoDecoderConfig) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&VideoDecoderConfig> for Any {
    fn from(s: &VideoDecoderConfig) -> Any {
        s.inner.clone()
    }
}

impl VideoDecoderConfig {
    pub fn codec(&self) -> DOMString {
        self.inner.get("codec").as_::<DOMString>()
    }

    pub fn set_codec(&mut self, value: &DOMString) {
        self.inner.set("codec", value);
    }
}
impl VideoDecoderConfig {
    pub fn description(&self) -> Any {
        self.inner.get("description").as_::<Any>()
    }

    pub fn set_description(&mut self, value: &Any) {
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

    pub fn set_color_space(&mut self, value: &VideoColorSpaceInit) {
        self.inner.set("colorSpace", value);
    }
}
impl VideoDecoderConfig {
    pub fn hardware_acceleration(&self) -> HardwareAcceleration {
        self.inner
            .get("hardwareAcceleration")
            .as_::<HardwareAcceleration>()
    }

    pub fn set_hardware_acceleration(&mut self, value: &HardwareAcceleration) {
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
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct VideoDecoderSupport {
    inner: Any,
}
impl FromVal for VideoDecoderSupport {
    fn from_val(v: &Any) -> Self {
        VideoDecoderSupport { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for VideoDecoderSupport {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for VideoDecoderSupport {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for VideoDecoderSupport {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for VideoDecoderSupport {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<VideoDecoderSupport> for Any {
    fn from(s: VideoDecoderSupport) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&VideoDecoderSupport> for Any {
    fn from(s: &VideoDecoderSupport) -> Any {
        s.inner.clone()
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

    pub fn set_config(&mut self, value: &VideoDecoderConfig) {
        self.inner.set("config", value);
    }
}
/// The VideoDecoder class.
/// [`VideoDecoder`](https://developer.mozilla.org/en-US/docs/Web/API/VideoDecoder)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct VideoDecoder {
    inner: EventTarget,
}
impl FromVal for VideoDecoder {
    fn from_val(v: &Any) -> Self {
        VideoDecoder {
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
impl core::ops::Deref for VideoDecoder {
    type Target = EventTarget;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for VideoDecoder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for VideoDecoder {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for VideoDecoder {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<VideoDecoder> for Any {
    fn from(s: VideoDecoder) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&VideoDecoder> for Any {
    fn from(s: &VideoDecoder) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(VideoDecoder);

impl VideoDecoder {
    /// The `new VideoDecoder(..)` constructor, creating a new VideoDecoder instance
    pub fn new(init: &Any) -> VideoDecoder {
        Self {
            inner: Any::global("VideoDecoder")
                .new(&[init.into()])
                .as_::<EventTarget>(),
        }
    }
}
impl VideoDecoder {
    /// Getter of the `state` attribute.
    /// [`VideoDecoder.state`](https://developer.mozilla.org/en-US/docs/Web/API/VideoDecoder/state)
    pub fn state(&self) -> CodecState {
        self.inner.get("state").as_::<CodecState>()
    }
}
impl VideoDecoder {
    /// Getter of the `decodeQueueSize` attribute.
    /// [`VideoDecoder.decodeQueueSize`](https://developer.mozilla.org/en-US/docs/Web/API/VideoDecoder/decodeQueueSize)
    pub fn decode_queue_size(&self) -> u32 {
        self.inner.get("decodeQueueSize").as_::<u32>()
    }
}
impl VideoDecoder {
    /// Getter of the `ondequeue` attribute.
    /// [`VideoDecoder.ondequeue`](https://developer.mozilla.org/en-US/docs/Web/API/VideoDecoder/ondequeue)
    pub fn ondequeue(&self) -> Any {
        self.inner.get("ondequeue").as_::<Any>()
    }

    /// Setter of the `ondequeue` attribute.
    /// [`VideoDecoder.ondequeue`](https://developer.mozilla.org/en-US/docs/Web/API/VideoDecoder/ondequeue)
    pub fn set_ondequeue(&mut self, value: &Any) {
        self.inner.set("ondequeue", value);
    }
}
impl VideoDecoder {
    /// The configure method.
    /// [`VideoDecoder.configure`](https://developer.mozilla.org/en-US/docs/Web/API/VideoDecoder/configure)
    pub fn configure(&self, config: &VideoDecoderConfig) -> Undefined {
        self.inner
            .call("configure", &[config.into()])
            .as_::<Undefined>()
    }
}
impl VideoDecoder {
    /// The decode method.
    /// [`VideoDecoder.decode`](https://developer.mozilla.org/en-US/docs/Web/API/VideoDecoder/decode)
    pub fn decode(&self, chunk: &EncodedVideoChunk) -> Undefined {
        self.inner
            .call("decode", &[chunk.into()])
            .as_::<Undefined>()
    }
}
impl VideoDecoder {
    /// The flush method.
    /// [`VideoDecoder.flush`](https://developer.mozilla.org/en-US/docs/Web/API/VideoDecoder/flush)
    pub fn flush(&self) -> Promise<Undefined> {
        self.inner.call("flush", &[]).as_::<Promise<Undefined>>()
    }
}
impl VideoDecoder {
    /// The reset method.
    /// [`VideoDecoder.reset`](https://developer.mozilla.org/en-US/docs/Web/API/VideoDecoder/reset)
    pub fn reset(&self) -> Undefined {
        self.inner.call("reset", &[]).as_::<Undefined>()
    }
}
impl VideoDecoder {
    /// The close method.
    /// [`VideoDecoder.close`](https://developer.mozilla.org/en-US/docs/Web/API/VideoDecoder/close)
    pub fn close(&self) -> Undefined {
        self.inner.call("close", &[]).as_::<Undefined>()
    }
}
impl VideoDecoder {
    /// The isConfigSupported method.
    /// [`VideoDecoder.isConfigSupported`](https://developer.mozilla.org/en-US/docs/Web/API/VideoDecoder/isConfigSupported)
    pub fn is_config_supported(config: &VideoDecoderConfig) -> Promise<VideoDecoderSupport> {
        Any::global("VideoDecoder")
            .call("isConfigSupported", &[config.into()])
            .as_::<Promise<VideoDecoderSupport>>()
    }
}

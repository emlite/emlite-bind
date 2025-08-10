use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct VideoEncoderConfig {
    inner: Any,
}
impl FromVal for VideoEncoderConfig {
    fn from_val(v: &Any) -> Self {
        VideoEncoderConfig { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for VideoEncoderConfig {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for VideoEncoderConfig {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for VideoEncoderConfig {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for VideoEncoderConfig {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<VideoEncoderConfig> for Any {
    fn from(s: VideoEncoderConfig) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&VideoEncoderConfig> for Any {
    fn from(s: &VideoEncoderConfig) -> Any {
        s.inner.clone()
    }
}

impl VideoEncoderConfig {
    pub fn codec(&self) -> JsString {
        self.inner.get("codec").as_::<JsString>()
    }

    pub fn set_codec(&mut self, value: &JsString) {
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

    pub fn set_hardware_acceleration(&mut self, value: &HardwareAcceleration) {
        self.inner.set("hardwareAcceleration", value);
    }
}
impl VideoEncoderConfig {
    pub fn alpha(&self) -> AlphaOption {
        self.inner.get("alpha").as_::<AlphaOption>()
    }

    pub fn set_alpha(&mut self, value: &AlphaOption) {
        self.inner.set("alpha", value);
    }
}
impl VideoEncoderConfig {
    pub fn scalability_mode(&self) -> JsString {
        self.inner.get("scalabilityMode").as_::<JsString>()
    }

    pub fn set_scalability_mode(&mut self, value: &JsString) {
        self.inner.set("scalabilityMode", value);
    }
}
impl VideoEncoderConfig {
    pub fn bitrate_mode(&self) -> VideoEncoderBitrateMode {
        self.inner
            .get("bitrateMode")
            .as_::<VideoEncoderBitrateMode>()
    }

    pub fn set_bitrate_mode(&mut self, value: &VideoEncoderBitrateMode) {
        self.inner.set("bitrateMode", value);
    }
}
impl VideoEncoderConfig {
    pub fn latency_mode(&self) -> LatencyMode {
        self.inner.get("latencyMode").as_::<LatencyMode>()
    }

    pub fn set_latency_mode(&mut self, value: &LatencyMode) {
        self.inner.set("latencyMode", value);
    }
}
impl VideoEncoderConfig {
    pub fn content_hint(&self) -> JsString {
        self.inner.get("contentHint").as_::<JsString>()
    }

    pub fn set_content_hint(&mut self, value: &JsString) {
        self.inner.set("contentHint", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct VideoEncoderEncodeOptions {
    inner: Any,
}
impl FromVal for VideoEncoderEncodeOptions {
    fn from_val(v: &Any) -> Self {
        VideoEncoderEncodeOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for VideoEncoderEncodeOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for VideoEncoderEncodeOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for VideoEncoderEncodeOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for VideoEncoderEncodeOptions {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<VideoEncoderEncodeOptions> for Any {
    fn from(s: VideoEncoderEncodeOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&VideoEncoderEncodeOptions> for Any {
    fn from(s: &VideoEncoderEncodeOptions) -> Any {
        s.inner.clone()
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
    inner: Any,
}
impl FromVal for VideoEncoderSupport {
    fn from_val(v: &Any) -> Self {
        VideoEncoderSupport { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for VideoEncoderSupport {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for VideoEncoderSupport {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for VideoEncoderSupport {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for VideoEncoderSupport {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<VideoEncoderSupport> for Any {
    fn from(s: VideoEncoderSupport) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&VideoEncoderSupport> for Any {
    fn from(s: &VideoEncoderSupport) -> Any {
        s.inner.clone()
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

    pub fn set_config(&mut self, value: &VideoEncoderConfig) {
        self.inner.set("config", value);
    }
}
/// The VideoEncoder class.
/// [`VideoEncoder`](https://developer.mozilla.org/en-US/docs/Web/API/VideoEncoder)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct VideoEncoder {
    inner: EventTarget,
}
impl FromVal for VideoEncoder {
    fn from_val(v: &Any) -> Self {
        VideoEncoder {
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
impl AsRef<Any> for VideoEncoder {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for VideoEncoder {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<VideoEncoder> for Any {
    fn from(s: VideoEncoder) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&VideoEncoder> for Any {
    fn from(s: &VideoEncoder) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(VideoEncoder);

impl VideoEncoder {
    /// The `new VideoEncoder(..)` constructor, creating a new VideoEncoder instance
    pub fn new(init: &VideoEncoderInit) -> VideoEncoder {
        Self {
            inner: Any::global("VideoEncoder")
                .new(&[init.into()])
                .as_::<EventTarget>(),
        }
    }
}
impl VideoEncoder {
    /// Getter of the `state` attribute.
    /// [`VideoEncoder.state`](https://developer.mozilla.org/en-US/docs/Web/API/VideoEncoder/state)
    pub fn state(&self) -> CodecState {
        self.inner.get("state").as_::<CodecState>()
    }
}
impl VideoEncoder {
    /// Getter of the `encodeQueueSize` attribute.
    /// [`VideoEncoder.encodeQueueSize`](https://developer.mozilla.org/en-US/docs/Web/API/VideoEncoder/encodeQueueSize)
    pub fn encode_queue_size(&self) -> u32 {
        self.inner.get("encodeQueueSize").as_::<u32>()
    }
}
impl VideoEncoder {
    /// Getter of the `ondequeue` attribute.
    /// [`VideoEncoder.ondequeue`](https://developer.mozilla.org/en-US/docs/Web/API/VideoEncoder/ondequeue)
    pub fn ondequeue(&self) -> Any {
        self.inner.get("ondequeue").as_::<Any>()
    }

    /// Setter of the `ondequeue` attribute.
    /// [`VideoEncoder.ondequeue`](https://developer.mozilla.org/en-US/docs/Web/API/VideoEncoder/ondequeue)
    pub fn set_ondequeue(&mut self, value: &Any) {
        self.inner.set("ondequeue", value);
    }
}
impl VideoEncoder {
    /// The configure method.
    /// [`VideoEncoder.configure`](https://developer.mozilla.org/en-US/docs/Web/API/VideoEncoder/configure)
    pub fn configure(&self, config: &VideoEncoderConfig) -> Undefined {
        self.inner
            .call("configure", &[config.into()])
            .as_::<Undefined>()
    }
}
impl VideoEncoder {
    /// The encode method.
    /// [`VideoEncoder.encode`](https://developer.mozilla.org/en-US/docs/Web/API/VideoEncoder/encode)
    pub fn encode0(&self, frame: &VideoFrame) -> Undefined {
        self.inner
            .call("encode", &[frame.into()])
            .as_::<Undefined>()
    }
    /// The encode method.
    /// [`VideoEncoder.encode`](https://developer.mozilla.org/en-US/docs/Web/API/VideoEncoder/encode)
    pub fn encode1(&self, frame: &VideoFrame, options: &VideoEncoderEncodeOptions) -> Undefined {
        self.inner
            .call("encode", &[frame.into(), options.into()])
            .as_::<Undefined>()
    }
}
impl VideoEncoder {
    /// The flush method.
    /// [`VideoEncoder.flush`](https://developer.mozilla.org/en-US/docs/Web/API/VideoEncoder/flush)
    pub fn flush(&self) -> Promise<Undefined> {
        self.inner.call("flush", &[]).as_::<Promise<Undefined>>()
    }
}
impl VideoEncoder {
    /// The reset method.
    /// [`VideoEncoder.reset`](https://developer.mozilla.org/en-US/docs/Web/API/VideoEncoder/reset)
    pub fn reset(&self) -> Undefined {
        self.inner.call("reset", &[]).as_::<Undefined>()
    }
}
impl VideoEncoder {
    /// The close method.
    /// [`VideoEncoder.close`](https://developer.mozilla.org/en-US/docs/Web/API/VideoEncoder/close)
    pub fn close(&self) -> Undefined {
        self.inner.call("close", &[]).as_::<Undefined>()
    }
}
impl VideoEncoder {
    /// The isConfigSupported method.
    /// [`VideoEncoder.isConfigSupported`](https://developer.mozilla.org/en-US/docs/Web/API/VideoEncoder/isConfigSupported)
    pub fn is_config_supported(config: &VideoEncoderConfig) -> Promise<VideoEncoderSupport> {
        Any::global("VideoEncoder")
            .call("isConfigSupported", &[config.into()])
            .as_::<Promise<VideoEncoderSupport>>()
    }
}

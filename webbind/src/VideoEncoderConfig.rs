use super::*;




/// The VideoEncoderConfig dictionary.
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
    /// Getter of the `codec` attribute.
    pub fn codec(&self) -> JsString {
        self.inner.get("codec").as_::<JsString>()
    }

    /// Setter of the `codec` attribute.
    pub fn set_codec(&mut self, value: &JsString) {
        self.inner.set("codec", value);
    }
}
impl VideoEncoderConfig {
    /// Getter of the `width` attribute.
    pub fn width(&self) -> u32 {
        self.inner.get("width").as_::<u32>()
    }

    /// Setter of the `width` attribute.
    pub fn set_width(&mut self, value: u32) {
        self.inner.set("width", value);
    }
}
impl VideoEncoderConfig {
    /// Getter of the `height` attribute.
    pub fn height(&self) -> u32 {
        self.inner.get("height").as_::<u32>()
    }

    /// Setter of the `height` attribute.
    pub fn set_height(&mut self, value: u32) {
        self.inner.set("height", value);
    }
}
impl VideoEncoderConfig {
    /// Getter of the `displayWidth` attribute.
    pub fn display_width(&self) -> u32 {
        self.inner.get("displayWidth").as_::<u32>()
    }

    /// Setter of the `displayWidth` attribute.
    pub fn set_display_width(&mut self, value: u32) {
        self.inner.set("displayWidth", value);
    }
}
impl VideoEncoderConfig {
    /// Getter of the `displayHeight` attribute.
    pub fn display_height(&self) -> u32 {
        self.inner.get("displayHeight").as_::<u32>()
    }

    /// Setter of the `displayHeight` attribute.
    pub fn set_display_height(&mut self, value: u32) {
        self.inner.set("displayHeight", value);
    }
}
impl VideoEncoderConfig {
    /// Getter of the `bitrate` attribute.
    pub fn bitrate(&self) -> u64 {
        self.inner.get("bitrate").as_::<u64>()
    }

    /// Setter of the `bitrate` attribute.
    pub fn set_bitrate(&mut self, value: u64) {
        self.inner.set("bitrate", value);
    }
}
impl VideoEncoderConfig {
    /// Getter of the `framerate` attribute.
    pub fn framerate(&self) -> f64 {
        self.inner.get("framerate").as_::<f64>()
    }

    /// Setter of the `framerate` attribute.
    pub fn set_framerate(&mut self, value: f64) {
        self.inner.set("framerate", value);
    }
}
impl VideoEncoderConfig {
    /// Getter of the `hardwareAcceleration` attribute.
    pub fn hardware_acceleration(&self) -> HardwareAcceleration {
        self.inner.get("hardwareAcceleration").as_::<HardwareAcceleration>()
    }

    /// Setter of the `hardwareAcceleration` attribute.
    pub fn set_hardware_acceleration(&mut self, value: &HardwareAcceleration) {
        self.inner.set("hardwareAcceleration", value);
    }
}
impl VideoEncoderConfig {
    /// Getter of the `alpha` attribute.
    pub fn alpha(&self) -> AlphaOption {
        self.inner.get("alpha").as_::<AlphaOption>()
    }

    /// Setter of the `alpha` attribute.
    pub fn set_alpha(&mut self, value: &AlphaOption) {
        self.inner.set("alpha", value);
    }
}
impl VideoEncoderConfig {
    /// Getter of the `scalabilityMode` attribute.
    pub fn scalability_mode(&self) -> JsString {
        self.inner.get("scalabilityMode").as_::<JsString>()
    }

    /// Setter of the `scalabilityMode` attribute.
    pub fn set_scalability_mode(&mut self, value: &JsString) {
        self.inner.set("scalabilityMode", value);
    }
}
impl VideoEncoderConfig {
    /// Getter of the `bitrateMode` attribute.
    pub fn bitrate_mode(&self) -> VideoEncoderBitrateMode {
        self.inner.get("bitrateMode").as_::<VideoEncoderBitrateMode>()
    }

    /// Setter of the `bitrateMode` attribute.
    pub fn set_bitrate_mode(&mut self, value: &VideoEncoderBitrateMode) {
        self.inner.set("bitrateMode", value);
    }
}
impl VideoEncoderConfig {
    /// Getter of the `latencyMode` attribute.
    pub fn latency_mode(&self) -> LatencyMode {
        self.inner.get("latencyMode").as_::<LatencyMode>()
    }

    /// Setter of the `latencyMode` attribute.
    pub fn set_latency_mode(&mut self, value: &LatencyMode) {
        self.inner.set("latencyMode", value);
    }
}
impl VideoEncoderConfig {
    /// Getter of the `contentHint` attribute.
    pub fn content_hint(&self) -> JsString {
        self.inner.get("contentHint").as_::<JsString>()
    }

    /// Setter of the `contentHint` attribute.
    pub fn set_content_hint(&mut self, value: &JsString) {
        self.inner.set("contentHint", value);
    }
}

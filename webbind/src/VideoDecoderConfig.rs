use super::*;

/// The VideoDecoderConfig dictionary.
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
    /// Getter of the `codec` attribute.
    pub fn codec(&self) -> JsString {
        self.inner.get("codec").as_::<JsString>()
    }

    /// Setter of the `codec` attribute.
    pub fn set_codec(&mut self, value: &JsString) {
        self.inner.set("codec", value);
    }
}
impl VideoDecoderConfig {
    /// Getter of the `description` attribute.
    pub fn description(&self) -> Any {
        self.inner.get("description").as_::<Any>()
    }

    /// Setter of the `description` attribute.
    pub fn set_description(&mut self, value: &Any) {
        self.inner.set("description", value);
    }
}
impl VideoDecoderConfig {
    /// Getter of the `codedWidth` attribute.
    pub fn coded_width(&self) -> u32 {
        self.inner.get("codedWidth").as_::<u32>()
    }

    /// Setter of the `codedWidth` attribute.
    pub fn set_coded_width(&mut self, value: u32) {
        self.inner.set("codedWidth", value);
    }
}
impl VideoDecoderConfig {
    /// Getter of the `codedHeight` attribute.
    pub fn coded_height(&self) -> u32 {
        self.inner.get("codedHeight").as_::<u32>()
    }

    /// Setter of the `codedHeight` attribute.
    pub fn set_coded_height(&mut self, value: u32) {
        self.inner.set("codedHeight", value);
    }
}
impl VideoDecoderConfig {
    /// Getter of the `displayAspectWidth` attribute.
    pub fn display_aspect_width(&self) -> u32 {
        self.inner.get("displayAspectWidth").as_::<u32>()
    }

    /// Setter of the `displayAspectWidth` attribute.
    pub fn set_display_aspect_width(&mut self, value: u32) {
        self.inner.set("displayAspectWidth", value);
    }
}
impl VideoDecoderConfig {
    /// Getter of the `displayAspectHeight` attribute.
    pub fn display_aspect_height(&self) -> u32 {
        self.inner.get("displayAspectHeight").as_::<u32>()
    }

    /// Setter of the `displayAspectHeight` attribute.
    pub fn set_display_aspect_height(&mut self, value: u32) {
        self.inner.set("displayAspectHeight", value);
    }
}
impl VideoDecoderConfig {
    /// Getter of the `colorSpace` attribute.
    pub fn color_space(&self) -> VideoColorSpaceInit {
        self.inner.get("colorSpace").as_::<VideoColorSpaceInit>()
    }

    /// Setter of the `colorSpace` attribute.
    pub fn set_color_space(&mut self, value: &VideoColorSpaceInit) {
        self.inner.set("colorSpace", value);
    }
}
impl VideoDecoderConfig {
    /// Getter of the `hardwareAcceleration` attribute.
    pub fn hardware_acceleration(&self) -> HardwareAcceleration {
        self.inner
            .get("hardwareAcceleration")
            .as_::<HardwareAcceleration>()
    }

    /// Setter of the `hardwareAcceleration` attribute.
    pub fn set_hardware_acceleration(&mut self, value: &HardwareAcceleration) {
        self.inner.set("hardwareAcceleration", value);
    }
}
impl VideoDecoderConfig {
    /// Getter of the `optimizeForLatency` attribute.
    pub fn optimize_for_latency(&self) -> bool {
        self.inner.get("optimizeForLatency").as_::<bool>()
    }

    /// Setter of the `optimizeForLatency` attribute.
    pub fn set_optimize_for_latency(&mut self, value: bool) {
        self.inner.set("optimizeForLatency", value);
    }
}
impl VideoDecoderConfig {
    /// Getter of the `rotation` attribute.
    pub fn rotation(&self) -> f64 {
        self.inner.get("rotation").as_::<f64>()
    }

    /// Setter of the `rotation` attribute.
    pub fn set_rotation(&mut self, value: f64) {
        self.inner.set("rotation", value);
    }
}
impl VideoDecoderConfig {
    /// Getter of the `flip` attribute.
    pub fn flip(&self) -> bool {
        self.inner.get("flip").as_::<bool>()
    }

    /// Setter of the `flip` attribute.
    pub fn set_flip(&mut self, value: bool) {
        self.inner.set("flip", value);
    }
}

use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct VideoConfiguration {
    inner: Any,
}
impl FromVal for VideoConfiguration {
    fn from_val(v: &Any) -> Self {
        VideoConfiguration { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for VideoConfiguration {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for VideoConfiguration {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for VideoConfiguration {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for VideoConfiguration {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<VideoConfiguration> for Any {
    fn from(s: VideoConfiguration) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&VideoConfiguration> for Any {
    fn from(s: &VideoConfiguration) -> Any {
        s.inner.clone()
    }
}

impl VideoConfiguration {
    pub fn content_type(&self) -> JsString {
        self.inner.get("contentType").as_::<JsString>()
    }

    pub fn set_content_type(&mut self, value: &JsString) {
        self.inner.set("contentType", value);
    }
}
impl VideoConfiguration {
    pub fn width(&self) -> u32 {
        self.inner.get("width").as_::<u32>()
    }

    pub fn set_width(&mut self, value: u32) {
        self.inner.set("width", value);
    }
}
impl VideoConfiguration {
    pub fn height(&self) -> u32 {
        self.inner.get("height").as_::<u32>()
    }

    pub fn set_height(&mut self, value: u32) {
        self.inner.set("height", value);
    }
}
impl VideoConfiguration {
    pub fn bitrate(&self) -> u64 {
        self.inner.get("bitrate").as_::<u64>()
    }

    pub fn set_bitrate(&mut self, value: u64) {
        self.inner.set("bitrate", value);
    }
}
impl VideoConfiguration {
    pub fn framerate(&self) -> f64 {
        self.inner.get("framerate").as_::<f64>()
    }

    pub fn set_framerate(&mut self, value: f64) {
        self.inner.set("framerate", value);
    }
}
impl VideoConfiguration {
    pub fn has_alpha_channel(&self) -> bool {
        self.inner.get("hasAlphaChannel").as_::<bool>()
    }

    pub fn set_has_alpha_channel(&mut self, value: bool) {
        self.inner.set("hasAlphaChannel", value);
    }
}
impl VideoConfiguration {
    pub fn hdr_metadata_type(&self) -> HdrMetadataType {
        self.inner.get("hdrMetadataType").as_::<HdrMetadataType>()
    }

    pub fn set_hdr_metadata_type(&mut self, value: &HdrMetadataType) {
        self.inner.set("hdrMetadataType", value);
    }
}
impl VideoConfiguration {
    pub fn color_gamut(&self) -> ColorGamut {
        self.inner.get("colorGamut").as_::<ColorGamut>()
    }

    pub fn set_color_gamut(&mut self, value: &ColorGamut) {
        self.inner.set("colorGamut", value);
    }
}
impl VideoConfiguration {
    pub fn transfer_function(&self) -> TransferFunction {
        self.inner.get("transferFunction").as_::<TransferFunction>()
    }

    pub fn set_transfer_function(&mut self, value: &TransferFunction) {
        self.inner.set("transferFunction", value);
    }
}
impl VideoConfiguration {
    pub fn scalability_mode(&self) -> JsString {
        self.inner.get("scalabilityMode").as_::<JsString>()
    }

    pub fn set_scalability_mode(&mut self, value: &JsString) {
        self.inner.set("scalabilityMode", value);
    }
}
impl VideoConfiguration {
    pub fn spatial_scalability(&self) -> bool {
        self.inner.get("spatialScalability").as_::<bool>()
    }

    pub fn set_spatial_scalability(&mut self, value: bool) {
        self.inner.set("spatialScalability", value);
    }
}

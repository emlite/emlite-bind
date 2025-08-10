use super::*;

/// The PhotoCapabilities dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct PhotoCapabilities {
    inner: Any,
}

impl FromVal for PhotoCapabilities {
    fn from_val(v: &Any) -> Self {
        PhotoCapabilities { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for PhotoCapabilities {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for PhotoCapabilities {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for PhotoCapabilities {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for PhotoCapabilities {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<PhotoCapabilities> for Any {
    fn from(s: PhotoCapabilities) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&PhotoCapabilities> for Any {
    fn from(s: &PhotoCapabilities) -> Any {
        s.inner.clone()
    }
}

impl PhotoCapabilities {
    /// Getter of the `redEyeReduction` attribute.
    pub fn red_eye_reduction(&self) -> RedEyeReduction {
        self.inner.get("redEyeReduction").as_::<RedEyeReduction>()
    }

    /// Setter of the `redEyeReduction` attribute.
    pub fn set_red_eye_reduction(&mut self, value: &RedEyeReduction) {
        self.inner.set("redEyeReduction", value);
    }
}
impl PhotoCapabilities {
    /// Getter of the `imageHeight` attribute.
    pub fn image_height(&self) -> MediaSettingsRange {
        self.inner.get("imageHeight").as_::<MediaSettingsRange>()
    }

    /// Setter of the `imageHeight` attribute.
    pub fn set_image_height(&mut self, value: &MediaSettingsRange) {
        self.inner.set("imageHeight", value);
    }
}
impl PhotoCapabilities {
    /// Getter of the `imageWidth` attribute.
    pub fn image_width(&self) -> MediaSettingsRange {
        self.inner.get("imageWidth").as_::<MediaSettingsRange>()
    }

    /// Setter of the `imageWidth` attribute.
    pub fn set_image_width(&mut self, value: &MediaSettingsRange) {
        self.inner.set("imageWidth", value);
    }
}
impl PhotoCapabilities {
    /// Getter of the `fillLightMode` attribute.
    pub fn fill_light_mode(&self) -> TypedArray<FillLightMode> {
        self.inner
            .get("fillLightMode")
            .as_::<TypedArray<FillLightMode>>()
    }

    /// Setter of the `fillLightMode` attribute.
    pub fn set_fill_light_mode(&mut self, value: &TypedArray<FillLightMode>) {
        self.inner.set("fillLightMode", value);
    }
}

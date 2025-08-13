use super::*;




/// The PhotoSettings dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct PhotoSettings {
    inner: Any,
}

impl FromVal for PhotoSettings {
    fn from_val(v: &Any) -> Self {
        PhotoSettings { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for PhotoSettings {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for PhotoSettings {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for PhotoSettings {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for PhotoSettings {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<PhotoSettings> for Any {
    fn from(s: PhotoSettings) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&PhotoSettings> for Any {
    fn from(s: &PhotoSettings) -> Any {
        s.inner.clone()
    }
}

impl PhotoSettings {
    /// Getter of the `fillLightMode` attribute.
    pub fn fill_light_mode(&self) -> FillLightMode {
        self.inner.get("fillLightMode").as_::<FillLightMode>()
    }

    /// Setter of the `fillLightMode` attribute.
    pub fn set_fill_light_mode(&mut self, value: &FillLightMode) {
        self.inner.set("fillLightMode", value);
    }
}
impl PhotoSettings {
    /// Getter of the `imageHeight` attribute.
    pub fn image_height(&self) -> f64 {
        self.inner.get("imageHeight").as_::<f64>()
    }

    /// Setter of the `imageHeight` attribute.
    pub fn set_image_height(&mut self, value: f64) {
        self.inner.set("imageHeight", value);
    }
}
impl PhotoSettings {
    /// Getter of the `imageWidth` attribute.
    pub fn image_width(&self) -> f64 {
        self.inner.get("imageWidth").as_::<f64>()
    }

    /// Setter of the `imageWidth` attribute.
    pub fn set_image_width(&mut self, value: f64) {
        self.inner.set("imageWidth", value);
    }
}
impl PhotoSettings {
    /// Getter of the `redEyeReduction` attribute.
    pub fn red_eye_reduction(&self) -> bool {
        self.inner.get("redEyeReduction").as_::<bool>()
    }

    /// Setter of the `redEyeReduction` attribute.
    pub fn set_red_eye_reduction(&mut self, value: bool) {
        self.inner.set("redEyeReduction", value);
    }
}

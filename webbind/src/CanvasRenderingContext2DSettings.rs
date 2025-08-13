use super::*;




/// The CanvasRenderingContext2DSettings dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct CanvasRenderingContext2DSettings {
    inner: Any,
}

impl FromVal for CanvasRenderingContext2DSettings {
    fn from_val(v: &Any) -> Self {
        CanvasRenderingContext2DSettings { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for CanvasRenderingContext2DSettings {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for CanvasRenderingContext2DSettings {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for CanvasRenderingContext2DSettings {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for CanvasRenderingContext2DSettings {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<CanvasRenderingContext2DSettings> for Any {
    fn from(s: CanvasRenderingContext2DSettings) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&CanvasRenderingContext2DSettings> for Any {
    fn from(s: &CanvasRenderingContext2DSettings) -> Any {
        s.inner.clone()
    }
}

impl CanvasRenderingContext2DSettings {
    /// Getter of the `alpha` attribute.
    pub fn alpha(&self) -> bool {
        self.inner.get("alpha").as_::<bool>()
    }

    /// Setter of the `alpha` attribute.
    pub fn set_alpha(&mut self, value: bool) {
        self.inner.set("alpha", value);
    }
}
impl CanvasRenderingContext2DSettings {
    /// Getter of the `desynchronized` attribute.
    pub fn desynchronized(&self) -> bool {
        self.inner.get("desynchronized").as_::<bool>()
    }

    /// Setter of the `desynchronized` attribute.
    pub fn set_desynchronized(&mut self, value: bool) {
        self.inner.set("desynchronized", value);
    }
}
impl CanvasRenderingContext2DSettings {
    /// Getter of the `colorSpace` attribute.
    pub fn color_space(&self) -> PredefinedColorSpace {
        self.inner.get("colorSpace").as_::<PredefinedColorSpace>()
    }

    /// Setter of the `colorSpace` attribute.
    pub fn set_color_space(&mut self, value: &PredefinedColorSpace) {
        self.inner.set("colorSpace", value);
    }
}
impl CanvasRenderingContext2DSettings {
    /// Getter of the `colorType` attribute.
    pub fn color_type(&self) -> CanvasColorType {
        self.inner.get("colorType").as_::<CanvasColorType>()
    }

    /// Setter of the `colorType` attribute.
    pub fn set_color_type(&mut self, value: &CanvasColorType) {
        self.inner.set("colorType", value);
    }
}
impl CanvasRenderingContext2DSettings {
    /// Getter of the `willReadFrequently` attribute.
    pub fn will_read_frequently(&self) -> bool {
        self.inner.get("willReadFrequently").as_::<bool>()
    }

    /// Setter of the `willReadFrequently` attribute.
    pub fn set_will_read_frequently(&mut self, value: bool) {
        self.inner.set("willReadFrequently", value);
    }
}

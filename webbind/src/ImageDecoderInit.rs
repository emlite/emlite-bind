use super::*;

/// The ImageDecoderInit dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct ImageDecoderInit {
    inner: Any,
}

impl FromVal for ImageDecoderInit {
    fn from_val(v: &Any) -> Self {
        ImageDecoderInit { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for ImageDecoderInit {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for ImageDecoderInit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for ImageDecoderInit {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for ImageDecoderInit {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<ImageDecoderInit> for Any {
    fn from(s: ImageDecoderInit) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&ImageDecoderInit> for Any {
    fn from(s: &ImageDecoderInit) -> Any {
        s.inner.clone()
    }
}

impl ImageDecoderInit {
    /// Getter of the `type` attribute.
    pub fn type_(&self) -> JsString {
        self.inner.get("type").as_::<JsString>()
    }

    /// Setter of the `type` attribute.
    pub fn set_type_(&mut self, value: &JsString) {
        self.inner.set("type", value);
    }
}
impl ImageDecoderInit {
    /// Getter of the `data` attribute.
    pub fn data(&self) -> Any {
        self.inner.get("data").as_::<Any>()
    }

    /// Setter of the `data` attribute.
    pub fn set_data(&mut self, value: &Any) {
        self.inner.set("data", value);
    }
}
impl ImageDecoderInit {
    /// Getter of the `colorSpaceConversion` attribute.
    pub fn color_space_conversion(&self) -> ColorSpaceConversion {
        self.inner
            .get("colorSpaceConversion")
            .as_::<ColorSpaceConversion>()
    }

    /// Setter of the `colorSpaceConversion` attribute.
    pub fn set_color_space_conversion(&mut self, value: &ColorSpaceConversion) {
        self.inner.set("colorSpaceConversion", value);
    }
}
impl ImageDecoderInit {
    /// Getter of the `desiredWidth` attribute.
    pub fn desired_width(&self) -> u32 {
        self.inner.get("desiredWidth").as_::<u32>()
    }

    /// Setter of the `desiredWidth` attribute.
    pub fn set_desired_width(&mut self, value: u32) {
        self.inner.set("desiredWidth", value);
    }
}
impl ImageDecoderInit {
    /// Getter of the `desiredHeight` attribute.
    pub fn desired_height(&self) -> u32 {
        self.inner.get("desiredHeight").as_::<u32>()
    }

    /// Setter of the `desiredHeight` attribute.
    pub fn set_desired_height(&mut self, value: u32) {
        self.inner.set("desiredHeight", value);
    }
}
impl ImageDecoderInit {
    /// Getter of the `preferAnimation` attribute.
    pub fn prefer_animation(&self) -> bool {
        self.inner.get("preferAnimation").as_::<bool>()
    }

    /// Setter of the `preferAnimation` attribute.
    pub fn set_prefer_animation(&mut self, value: bool) {
        self.inner.set("preferAnimation", value);
    }
}
impl ImageDecoderInit {
    /// Getter of the `transfer` attribute.
    pub fn transfer(&self) -> TypedArray<ArrayBuffer> {
        self.inner.get("transfer").as_::<TypedArray<ArrayBuffer>>()
    }

    /// Setter of the `transfer` attribute.
    pub fn set_transfer(&mut self, value: &TypedArray<ArrayBuffer>) {
        self.inner.set("transfer", value);
    }
}

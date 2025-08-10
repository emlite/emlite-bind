use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct FontFaceDescriptors {
    inner: Any,
}
impl FromVal for FontFaceDescriptors {
    fn from_val(v: &Any) -> Self {
        FontFaceDescriptors { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for FontFaceDescriptors {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for FontFaceDescriptors {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for FontFaceDescriptors {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for FontFaceDescriptors {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<FontFaceDescriptors> for Any {
    fn from(s: FontFaceDescriptors) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&FontFaceDescriptors> for Any {
    fn from(s: &FontFaceDescriptors) -> Any {
        s.inner.clone()
    }
}

impl FontFaceDescriptors {
    pub fn style(&self) -> JsString {
        self.inner.get("style").as_::<JsString>()
    }

    pub fn set_style(&mut self, value: &JsString) {
        self.inner.set("style", value);
    }
}
impl FontFaceDescriptors {
    pub fn weight(&self) -> JsString {
        self.inner.get("weight").as_::<JsString>()
    }

    pub fn set_weight(&mut self, value: &JsString) {
        self.inner.set("weight", value);
    }
}
impl FontFaceDescriptors {
    pub fn stretch(&self) -> JsString {
        self.inner.get("stretch").as_::<JsString>()
    }

    pub fn set_stretch(&mut self, value: &JsString) {
        self.inner.set("stretch", value);
    }
}
impl FontFaceDescriptors {
    pub fn unicode_range(&self) -> JsString {
        self.inner.get("unicodeRange").as_::<JsString>()
    }

    pub fn set_unicode_range(&mut self, value: &JsString) {
        self.inner.set("unicodeRange", value);
    }
}
impl FontFaceDescriptors {
    pub fn feature_settings(&self) -> JsString {
        self.inner.get("featureSettings").as_::<JsString>()
    }

    pub fn set_feature_settings(&mut self, value: &JsString) {
        self.inner.set("featureSettings", value);
    }
}
impl FontFaceDescriptors {
    pub fn variation_settings(&self) -> JsString {
        self.inner.get("variationSettings").as_::<JsString>()
    }

    pub fn set_variation_settings(&mut self, value: &JsString) {
        self.inner.set("variationSettings", value);
    }
}
impl FontFaceDescriptors {
    pub fn display(&self) -> JsString {
        self.inner.get("display").as_::<JsString>()
    }

    pub fn set_display(&mut self, value: &JsString) {
        self.inner.set("display", value);
    }
}
impl FontFaceDescriptors {
    pub fn ascent_override(&self) -> JsString {
        self.inner.get("ascentOverride").as_::<JsString>()
    }

    pub fn set_ascent_override(&mut self, value: &JsString) {
        self.inner.set("ascentOverride", value);
    }
}
impl FontFaceDescriptors {
    pub fn descent_override(&self) -> JsString {
        self.inner.get("descentOverride").as_::<JsString>()
    }

    pub fn set_descent_override(&mut self, value: &JsString) {
        self.inner.set("descentOverride", value);
    }
}
impl FontFaceDescriptors {
    pub fn line_gap_override(&self) -> JsString {
        self.inner.get("lineGapOverride").as_::<JsString>()
    }

    pub fn set_line_gap_override(&mut self, value: &JsString) {
        self.inner.set("lineGapOverride", value);
    }
}

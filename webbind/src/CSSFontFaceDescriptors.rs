use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct CSSFontFaceDescriptors {
    inner: CSSStyleDeclaration,
}
impl FromVal for CSSFontFaceDescriptors {
    fn from_val(v: &emlite::Val) -> Self {
        CSSFontFaceDescriptors {
            inner: CSSStyleDeclaration::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for CSSFontFaceDescriptors {
    type Target = CSSStyleDeclaration;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for CSSFontFaceDescriptors {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<CSSFontFaceDescriptors> for emlite::Val {
    fn from(x: CSSFontFaceDescriptors) -> emlite::Val {
        let handle = x.inner.as_handle();
        core::mem::forget(x);
        emlite::Val::take_ownership(handle)
    }
}

impl CSSFontFaceDescriptors {
    pub fn src(&self) -> jsbind::CSSOMString {
        self.inner.get("src").as_::<jsbind::CSSOMString>()
    }

    pub fn set_src(&mut self, value: jsbind::CSSOMString) {
        self.inner.set("src", value);
    }
}
impl CSSFontFaceDescriptors {
    pub fn font_family(&self) -> jsbind::CSSOMString {
        self.inner.get("fontFamily").as_::<jsbind::CSSOMString>()
    }

    pub fn set_font_family(&mut self, value: jsbind::CSSOMString) {
        self.inner.set("fontFamily", value);
    }
}

impl CSSFontFaceDescriptors {
    pub fn font_style(&self) -> jsbind::CSSOMString {
        self.inner.get("fontStyle").as_::<jsbind::CSSOMString>()
    }

    pub fn set_font_style(&mut self, value: jsbind::CSSOMString) {
        self.inner.set("fontStyle", value);
    }
}

impl CSSFontFaceDescriptors {
    pub fn font_weight(&self) -> jsbind::CSSOMString {
        self.inner.get("fontWeight").as_::<jsbind::CSSOMString>()
    }

    pub fn set_font_weight(&mut self, value: jsbind::CSSOMString) {
        self.inner.set("fontWeight", value);
    }
}

impl CSSFontFaceDescriptors {
    pub fn font_stretch(&self) -> jsbind::CSSOMString {
        self.inner.get("fontStretch").as_::<jsbind::CSSOMString>()
    }

    pub fn set_font_stretch(&mut self, value: jsbind::CSSOMString) {
        self.inner.set("fontStretch", value);
    }
}

impl CSSFontFaceDescriptors {
    pub fn font_width(&self) -> jsbind::CSSOMString {
        self.inner.get("fontWidth").as_::<jsbind::CSSOMString>()
    }

    pub fn set_font_width(&mut self, value: jsbind::CSSOMString) {
        self.inner.set("fontWidth", value);
    }
}

impl CSSFontFaceDescriptors {
    pub fn font_size(&self) -> jsbind::CSSOMString {
        self.inner.get("fontSize").as_::<jsbind::CSSOMString>()
    }

    pub fn set_font_size(&mut self, value: jsbind::CSSOMString) {
        self.inner.set("fontSize", value);
    }
}

impl CSSFontFaceDescriptors {
    pub fn size_adjust(&self) -> jsbind::CSSOMString {
        self.inner.get("sizeAdjust").as_::<jsbind::CSSOMString>()
    }

    pub fn set_size_adjust(&mut self, value: jsbind::CSSOMString) {
        self.inner.set("sizeAdjust", value);
    }
}

impl CSSFontFaceDescriptors {
    pub fn unicode_range(&self) -> jsbind::CSSOMString {
        self.inner.get("unicodeRange").as_::<jsbind::CSSOMString>()
    }

    pub fn set_unicode_range(&mut self, value: jsbind::CSSOMString) {
        self.inner.set("unicodeRange", value);
    }
}

impl CSSFontFaceDescriptors {
    pub fn font_feature_settings(&self) -> jsbind::CSSOMString {
        self.inner
            .get("fontFeatureSettings")
            .as_::<jsbind::CSSOMString>()
    }

    pub fn set_font_feature_settings(&mut self, value: jsbind::CSSOMString) {
        self.inner.set("fontFeatureSettings", value);
    }
}

impl CSSFontFaceDescriptors {
    pub fn font_variation_settings(&self) -> jsbind::CSSOMString {
        self.inner
            .get("fontVariationSettings")
            .as_::<jsbind::CSSOMString>()
    }

    pub fn set_font_variation_settings(&mut self, value: jsbind::CSSOMString) {
        self.inner.set("fontVariationSettings", value);
    }
}

impl CSSFontFaceDescriptors {
    pub fn font_named_instance(&self) -> jsbind::CSSOMString {
        self.inner
            .get("fontNamedInstance")
            .as_::<jsbind::CSSOMString>()
    }

    pub fn set_font_named_instance(&mut self, value: jsbind::CSSOMString) {
        self.inner.set("fontNamedInstance", value);
    }
}

impl CSSFontFaceDescriptors {
    pub fn font_display(&self) -> jsbind::CSSOMString {
        self.inner.get("fontDisplay").as_::<jsbind::CSSOMString>()
    }

    pub fn set_font_display(&mut self, value: jsbind::CSSOMString) {
        self.inner.set("fontDisplay", value);
    }
}

impl CSSFontFaceDescriptors {
    pub fn font_language_override(&self) -> jsbind::CSSOMString {
        self.inner
            .get("fontLanguageOverride")
            .as_::<jsbind::CSSOMString>()
    }

    pub fn set_font_language_override(&mut self, value: jsbind::CSSOMString) {
        self.inner.set("fontLanguageOverride", value);
    }
}

impl CSSFontFaceDescriptors {
    pub fn ascent_override(&self) -> jsbind::CSSOMString {
        self.inner
            .get("ascentOverride")
            .as_::<jsbind::CSSOMString>()
    }

    pub fn set_ascent_override(&mut self, value: jsbind::CSSOMString) {
        self.inner.set("ascentOverride", value);
    }
}

impl CSSFontFaceDescriptors {
    pub fn descent_override(&self) -> jsbind::CSSOMString {
        self.inner
            .get("descentOverride")
            .as_::<jsbind::CSSOMString>()
    }

    pub fn set_descent_override(&mut self, value: jsbind::CSSOMString) {
        self.inner.set("descentOverride", value);
    }
}

impl CSSFontFaceDescriptors {
    pub fn line_gap_override(&self) -> jsbind::CSSOMString {
        self.inner
            .get("lineGapOverride")
            .as_::<jsbind::CSSOMString>()
    }

    pub fn set_line_gap_override(&mut self, value: jsbind::CSSOMString) {
        self.inner.set("lineGapOverride", value);
    }
}

impl CSSFontFaceDescriptors {
    pub fn superscript_position_override(&self) -> jsbind::CSSOMString {
        self.inner
            .get("superscriptPositionOverride")
            .as_::<jsbind::CSSOMString>()
    }

    pub fn set_superscript_position_override(&mut self, value: jsbind::CSSOMString) {
        self.inner.set("superscriptPositionOverride", value);
    }
}

impl CSSFontFaceDescriptors {
    pub fn subscript_position_override(&self) -> jsbind::CSSOMString {
        self.inner
            .get("subscriptPositionOverride")
            .as_::<jsbind::CSSOMString>()
    }

    pub fn set_subscript_position_override(&mut self, value: jsbind::CSSOMString) {
        self.inner.set("subscriptPositionOverride", value);
    }
}

impl CSSFontFaceDescriptors {
    pub fn superscript_size_override(&self) -> jsbind::CSSOMString {
        self.inner
            .get("superscriptSizeOverride")
            .as_::<jsbind::CSSOMString>()
    }

    pub fn set_superscript_size_override(&mut self, value: jsbind::CSSOMString) {
        self.inner.set("superscriptSizeOverride", value);
    }
}

impl CSSFontFaceDescriptors {
    pub fn subscript_size_override(&self) -> jsbind::CSSOMString {
        self.inner
            .get("subscriptSizeOverride")
            .as_::<jsbind::CSSOMString>()
    }

    pub fn set_subscript_size_override(&mut self, value: jsbind::CSSOMString) {
        self.inner.set("subscriptSizeOverride", value);
    }
}

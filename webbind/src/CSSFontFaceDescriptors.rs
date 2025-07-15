use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct CSSFontFaceDescriptors {
    inner: CSSStyleDeclaration,
}

jsbind::utils::impl_dyn_cast!(CSSFontFaceDescriptors);

impl AsRef<emlite::Val> for CSSFontFaceDescriptors {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}

impl AsMut<emlite::Val> for CSSFontFaceDescriptors {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
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

impl From<&CSSFontFaceDescriptors> for emlite::Val {
    fn from(x: &CSSFontFaceDescriptors) -> emlite::Val {
        x.inner.clone().into()
    }
}

impl CSSFontFaceDescriptors {
    pub fn src(&self) -> CSSOMString {
        self.inner.get("src").as_::<CSSOMString>()
    }

    pub fn set_src(&mut self, value: CSSOMString) {
        self.inner.set("src", value);
    }
}
impl CSSFontFaceDescriptors {
    pub fn font_family(&self) -> CSSOMString {
        self.inner.get("fontFamily").as_::<CSSOMString>()
    }

    pub fn set_font_family(&mut self, value: CSSOMString) {
        self.inner.set("fontFamily", value);
    }
}

impl CSSFontFaceDescriptors {
    pub fn font_style(&self) -> CSSOMString {
        self.inner.get("fontStyle").as_::<CSSOMString>()
    }

    pub fn set_font_style(&mut self, value: CSSOMString) {
        self.inner.set("fontStyle", value);
    }
}

impl CSSFontFaceDescriptors {
    pub fn font_weight(&self) -> CSSOMString {
        self.inner.get("fontWeight").as_::<CSSOMString>()
    }

    pub fn set_font_weight(&mut self, value: CSSOMString) {
        self.inner.set("fontWeight", value);
    }
}

impl CSSFontFaceDescriptors {
    pub fn font_stretch(&self) -> CSSOMString {
        self.inner.get("fontStretch").as_::<CSSOMString>()
    }

    pub fn set_font_stretch(&mut self, value: CSSOMString) {
        self.inner.set("fontStretch", value);
    }
}

impl CSSFontFaceDescriptors {
    pub fn font_width(&self) -> CSSOMString {
        self.inner.get("fontWidth").as_::<CSSOMString>()
    }

    pub fn set_font_width(&mut self, value: CSSOMString) {
        self.inner.set("fontWidth", value);
    }
}

impl CSSFontFaceDescriptors {
    pub fn font_size(&self) -> CSSOMString {
        self.inner.get("fontSize").as_::<CSSOMString>()
    }

    pub fn set_font_size(&mut self, value: CSSOMString) {
        self.inner.set("fontSize", value);
    }
}

impl CSSFontFaceDescriptors {
    pub fn size_adjust(&self) -> CSSOMString {
        self.inner.get("sizeAdjust").as_::<CSSOMString>()
    }

    pub fn set_size_adjust(&mut self, value: CSSOMString) {
        self.inner.set("sizeAdjust", value);
    }
}

impl CSSFontFaceDescriptors {
    pub fn unicode_range(&self) -> CSSOMString {
        self.inner.get("unicodeRange").as_::<CSSOMString>()
    }

    pub fn set_unicode_range(&mut self, value: CSSOMString) {
        self.inner.set("unicodeRange", value);
    }
}

impl CSSFontFaceDescriptors {
    pub fn font_feature_settings(&self) -> CSSOMString {
        self.inner.get("fontFeatureSettings").as_::<CSSOMString>()
    }

    pub fn set_font_feature_settings(&mut self, value: CSSOMString) {
        self.inner.set("fontFeatureSettings", value);
    }
}

impl CSSFontFaceDescriptors {
    pub fn font_variation_settings(&self) -> CSSOMString {
        self.inner.get("fontVariationSettings").as_::<CSSOMString>()
    }

    pub fn set_font_variation_settings(&mut self, value: CSSOMString) {
        self.inner.set("fontVariationSettings", value);
    }
}

impl CSSFontFaceDescriptors {
    pub fn font_named_instance(&self) -> CSSOMString {
        self.inner.get("fontNamedInstance").as_::<CSSOMString>()
    }

    pub fn set_font_named_instance(&mut self, value: CSSOMString) {
        self.inner.set("fontNamedInstance", value);
    }
}

impl CSSFontFaceDescriptors {
    pub fn font_display(&self) -> CSSOMString {
        self.inner.get("fontDisplay").as_::<CSSOMString>()
    }

    pub fn set_font_display(&mut self, value: CSSOMString) {
        self.inner.set("fontDisplay", value);
    }
}

impl CSSFontFaceDescriptors {
    pub fn font_language_override(&self) -> CSSOMString {
        self.inner.get("fontLanguageOverride").as_::<CSSOMString>()
    }

    pub fn set_font_language_override(&mut self, value: CSSOMString) {
        self.inner.set("fontLanguageOverride", value);
    }
}

impl CSSFontFaceDescriptors {
    pub fn ascent_override(&self) -> CSSOMString {
        self.inner.get("ascentOverride").as_::<CSSOMString>()
    }

    pub fn set_ascent_override(&mut self, value: CSSOMString) {
        self.inner.set("ascentOverride", value);
    }
}

impl CSSFontFaceDescriptors {
    pub fn descent_override(&self) -> CSSOMString {
        self.inner.get("descentOverride").as_::<CSSOMString>()
    }

    pub fn set_descent_override(&mut self, value: CSSOMString) {
        self.inner.set("descentOverride", value);
    }
}

impl CSSFontFaceDescriptors {
    pub fn line_gap_override(&self) -> CSSOMString {
        self.inner.get("lineGapOverride").as_::<CSSOMString>()
    }

    pub fn set_line_gap_override(&mut self, value: CSSOMString) {
        self.inner.set("lineGapOverride", value);
    }
}

impl CSSFontFaceDescriptors {
    pub fn superscript_position_override(&self) -> CSSOMString {
        self.inner
            .get("superscriptPositionOverride")
            .as_::<CSSOMString>()
    }

    pub fn set_superscript_position_override(&mut self, value: CSSOMString) {
        self.inner.set("superscriptPositionOverride", value);
    }
}

impl CSSFontFaceDescriptors {
    pub fn subscript_position_override(&self) -> CSSOMString {
        self.inner
            .get("subscriptPositionOverride")
            .as_::<CSSOMString>()
    }

    pub fn set_subscript_position_override(&mut self, value: CSSOMString) {
        self.inner.set("subscriptPositionOverride", value);
    }
}

impl CSSFontFaceDescriptors {
    pub fn superscript_size_override(&self) -> CSSOMString {
        self.inner
            .get("superscriptSizeOverride")
            .as_::<CSSOMString>()
    }

    pub fn set_superscript_size_override(&mut self, value: CSSOMString) {
        self.inner.set("superscriptSizeOverride", value);
    }
}

impl CSSFontFaceDescriptors {
    pub fn subscript_size_override(&self) -> CSSOMString {
        self.inner.get("subscriptSizeOverride").as_::<CSSOMString>()
    }

    pub fn set_subscript_size_override(&mut self, value: CSSOMString) {
        self.inner.set("subscriptSizeOverride", value);
    }
}

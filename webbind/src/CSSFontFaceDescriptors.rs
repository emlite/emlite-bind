use super::*;

/// The CSSFontFaceDescriptors class.
/// [`CSSFontFaceDescriptors`](https://developer.mozilla.org/en-US/docs/Web/API/CSSFontFaceDescriptors)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct CSSFontFaceDescriptors {
    inner: CSSStyleDeclaration,
}
impl FromVal for CSSFontFaceDescriptors {
    fn from_val(v: &Any) -> Self {
        CSSFontFaceDescriptors {
            inner: CSSStyleDeclaration::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
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
impl AsRef<Any> for CSSFontFaceDescriptors {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for CSSFontFaceDescriptors {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<CSSFontFaceDescriptors> for Any {
    fn from(s: CSSFontFaceDescriptors) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&CSSFontFaceDescriptors> for Any {
    fn from(s: &CSSFontFaceDescriptors) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(CSSFontFaceDescriptors);

impl CSSFontFaceDescriptors {
    /// Getter of the `src` attribute.
    /// [`CSSFontFaceDescriptors.src`](https://developer.mozilla.org/en-US/docs/Web/API/CSSFontFaceDescriptors/src)
    pub fn src(&self) -> CSSOMString {
        self.inner.get("src").as_::<CSSOMString>()
    }

    /// Setter of the `src` attribute.
    /// [`CSSFontFaceDescriptors.src`](https://developer.mozilla.org/en-US/docs/Web/API/CSSFontFaceDescriptors/src)
    pub fn set_src(&mut self, value: &CSSOMString) {
        self.inner.set("src", value);
    }
}
impl CSSFontFaceDescriptors {
    /// Getter of the `fontFamily` attribute.
    /// [`CSSFontFaceDescriptors.fontFamily`](https://developer.mozilla.org/en-US/docs/Web/API/CSSFontFaceDescriptors/fontFamily)
    pub fn font_family(&self) -> CSSOMString {
        self.inner.get("fontFamily").as_::<CSSOMString>()
    }

    /// Setter of the `fontFamily` attribute.
    /// [`CSSFontFaceDescriptors.fontFamily`](https://developer.mozilla.org/en-US/docs/Web/API/CSSFontFaceDescriptors/fontFamily)
    pub fn set_font_family(&mut self, value: &CSSOMString) {
        self.inner.set("fontFamily", value);
    }
}
impl CSSFontFaceDescriptors {
    /// Getter of the `fontStyle` attribute.
    /// [`CSSFontFaceDescriptors.fontStyle`](https://developer.mozilla.org/en-US/docs/Web/API/CSSFontFaceDescriptors/fontStyle)
    pub fn font_style(&self) -> CSSOMString {
        self.inner.get("fontStyle").as_::<CSSOMString>()
    }

    /// Setter of the `fontStyle` attribute.
    /// [`CSSFontFaceDescriptors.fontStyle`](https://developer.mozilla.org/en-US/docs/Web/API/CSSFontFaceDescriptors/fontStyle)
    pub fn set_font_style(&mut self, value: &CSSOMString) {
        self.inner.set("fontStyle", value);
    }
}
impl CSSFontFaceDescriptors {
    /// Getter of the `fontWeight` attribute.
    /// [`CSSFontFaceDescriptors.fontWeight`](https://developer.mozilla.org/en-US/docs/Web/API/CSSFontFaceDescriptors/fontWeight)
    pub fn font_weight(&self) -> CSSOMString {
        self.inner.get("fontWeight").as_::<CSSOMString>()
    }

    /// Setter of the `fontWeight` attribute.
    /// [`CSSFontFaceDescriptors.fontWeight`](https://developer.mozilla.org/en-US/docs/Web/API/CSSFontFaceDescriptors/fontWeight)
    pub fn set_font_weight(&mut self, value: &CSSOMString) {
        self.inner.set("fontWeight", value);
    }
}
impl CSSFontFaceDescriptors {
    /// Getter of the `fontStretch` attribute.
    /// [`CSSFontFaceDescriptors.fontStretch`](https://developer.mozilla.org/en-US/docs/Web/API/CSSFontFaceDescriptors/fontStretch)
    pub fn font_stretch(&self) -> CSSOMString {
        self.inner.get("fontStretch").as_::<CSSOMString>()
    }

    /// Setter of the `fontStretch` attribute.
    /// [`CSSFontFaceDescriptors.fontStretch`](https://developer.mozilla.org/en-US/docs/Web/API/CSSFontFaceDescriptors/fontStretch)
    pub fn set_font_stretch(&mut self, value: &CSSOMString) {
        self.inner.set("fontStretch", value);
    }
}
impl CSSFontFaceDescriptors {
    /// Getter of the `fontWidth` attribute.
    /// [`CSSFontFaceDescriptors.fontWidth`](https://developer.mozilla.org/en-US/docs/Web/API/CSSFontFaceDescriptors/fontWidth)
    pub fn font_width(&self) -> CSSOMString {
        self.inner.get("fontWidth").as_::<CSSOMString>()
    }

    /// Setter of the `fontWidth` attribute.
    /// [`CSSFontFaceDescriptors.fontWidth`](https://developer.mozilla.org/en-US/docs/Web/API/CSSFontFaceDescriptors/fontWidth)
    pub fn set_font_width(&mut self, value: &CSSOMString) {
        self.inner.set("fontWidth", value);
    }
}
impl CSSFontFaceDescriptors {
    /// Getter of the `fontSize` attribute.
    /// [`CSSFontFaceDescriptors.fontSize`](https://developer.mozilla.org/en-US/docs/Web/API/CSSFontFaceDescriptors/fontSize)
    pub fn font_size(&self) -> CSSOMString {
        self.inner.get("fontSize").as_::<CSSOMString>()
    }

    /// Setter of the `fontSize` attribute.
    /// [`CSSFontFaceDescriptors.fontSize`](https://developer.mozilla.org/en-US/docs/Web/API/CSSFontFaceDescriptors/fontSize)
    pub fn set_font_size(&mut self, value: &CSSOMString) {
        self.inner.set("fontSize", value);
    }
}
impl CSSFontFaceDescriptors {
    /// Getter of the `sizeAdjust` attribute.
    /// [`CSSFontFaceDescriptors.sizeAdjust`](https://developer.mozilla.org/en-US/docs/Web/API/CSSFontFaceDescriptors/sizeAdjust)
    pub fn size_adjust(&self) -> CSSOMString {
        self.inner.get("sizeAdjust").as_::<CSSOMString>()
    }

    /// Setter of the `sizeAdjust` attribute.
    /// [`CSSFontFaceDescriptors.sizeAdjust`](https://developer.mozilla.org/en-US/docs/Web/API/CSSFontFaceDescriptors/sizeAdjust)
    pub fn set_size_adjust(&mut self, value: &CSSOMString) {
        self.inner.set("sizeAdjust", value);
    }
}
impl CSSFontFaceDescriptors {
    /// Getter of the `unicodeRange` attribute.
    /// [`CSSFontFaceDescriptors.unicodeRange`](https://developer.mozilla.org/en-US/docs/Web/API/CSSFontFaceDescriptors/unicodeRange)
    pub fn unicode_range(&self) -> CSSOMString {
        self.inner.get("unicodeRange").as_::<CSSOMString>()
    }

    /// Setter of the `unicodeRange` attribute.
    /// [`CSSFontFaceDescriptors.unicodeRange`](https://developer.mozilla.org/en-US/docs/Web/API/CSSFontFaceDescriptors/unicodeRange)
    pub fn set_unicode_range(&mut self, value: &CSSOMString) {
        self.inner.set("unicodeRange", value);
    }
}
impl CSSFontFaceDescriptors {
    /// Getter of the `fontFeatureSettings` attribute.
    /// [`CSSFontFaceDescriptors.fontFeatureSettings`](https://developer.mozilla.org/en-US/docs/Web/API/CSSFontFaceDescriptors/fontFeatureSettings)
    pub fn font_feature_settings(&self) -> CSSOMString {
        self.inner.get("fontFeatureSettings").as_::<CSSOMString>()
    }

    /// Setter of the `fontFeatureSettings` attribute.
    /// [`CSSFontFaceDescriptors.fontFeatureSettings`](https://developer.mozilla.org/en-US/docs/Web/API/CSSFontFaceDescriptors/fontFeatureSettings)
    pub fn set_font_feature_settings(&mut self, value: &CSSOMString) {
        self.inner.set("fontFeatureSettings", value);
    }
}
impl CSSFontFaceDescriptors {
    /// Getter of the `fontVariationSettings` attribute.
    /// [`CSSFontFaceDescriptors.fontVariationSettings`](https://developer.mozilla.org/en-US/docs/Web/API/CSSFontFaceDescriptors/fontVariationSettings)
    pub fn font_variation_settings(&self) -> CSSOMString {
        self.inner.get("fontVariationSettings").as_::<CSSOMString>()
    }

    /// Setter of the `fontVariationSettings` attribute.
    /// [`CSSFontFaceDescriptors.fontVariationSettings`](https://developer.mozilla.org/en-US/docs/Web/API/CSSFontFaceDescriptors/fontVariationSettings)
    pub fn set_font_variation_settings(&mut self, value: &CSSOMString) {
        self.inner.set("fontVariationSettings", value);
    }
}
impl CSSFontFaceDescriptors {
    /// Getter of the `fontNamedInstance` attribute.
    /// [`CSSFontFaceDescriptors.fontNamedInstance`](https://developer.mozilla.org/en-US/docs/Web/API/CSSFontFaceDescriptors/fontNamedInstance)
    pub fn font_named_instance(&self) -> CSSOMString {
        self.inner.get("fontNamedInstance").as_::<CSSOMString>()
    }

    /// Setter of the `fontNamedInstance` attribute.
    /// [`CSSFontFaceDescriptors.fontNamedInstance`](https://developer.mozilla.org/en-US/docs/Web/API/CSSFontFaceDescriptors/fontNamedInstance)
    pub fn set_font_named_instance(&mut self, value: &CSSOMString) {
        self.inner.set("fontNamedInstance", value);
    }
}
impl CSSFontFaceDescriptors {
    /// Getter of the `fontDisplay` attribute.
    /// [`CSSFontFaceDescriptors.fontDisplay`](https://developer.mozilla.org/en-US/docs/Web/API/CSSFontFaceDescriptors/fontDisplay)
    pub fn font_display(&self) -> CSSOMString {
        self.inner.get("fontDisplay").as_::<CSSOMString>()
    }

    /// Setter of the `fontDisplay` attribute.
    /// [`CSSFontFaceDescriptors.fontDisplay`](https://developer.mozilla.org/en-US/docs/Web/API/CSSFontFaceDescriptors/fontDisplay)
    pub fn set_font_display(&mut self, value: &CSSOMString) {
        self.inner.set("fontDisplay", value);
    }
}
impl CSSFontFaceDescriptors {
    /// Getter of the `fontLanguageOverride` attribute.
    /// [`CSSFontFaceDescriptors.fontLanguageOverride`](https://developer.mozilla.org/en-US/docs/Web/API/CSSFontFaceDescriptors/fontLanguageOverride)
    pub fn font_language_override(&self) -> CSSOMString {
        self.inner.get("fontLanguageOverride").as_::<CSSOMString>()
    }

    /// Setter of the `fontLanguageOverride` attribute.
    /// [`CSSFontFaceDescriptors.fontLanguageOverride`](https://developer.mozilla.org/en-US/docs/Web/API/CSSFontFaceDescriptors/fontLanguageOverride)
    pub fn set_font_language_override(&mut self, value: &CSSOMString) {
        self.inner.set("fontLanguageOverride", value);
    }
}
impl CSSFontFaceDescriptors {
    /// Getter of the `ascentOverride` attribute.
    /// [`CSSFontFaceDescriptors.ascentOverride`](https://developer.mozilla.org/en-US/docs/Web/API/CSSFontFaceDescriptors/ascentOverride)
    pub fn ascent_override(&self) -> CSSOMString {
        self.inner.get("ascentOverride").as_::<CSSOMString>()
    }

    /// Setter of the `ascentOverride` attribute.
    /// [`CSSFontFaceDescriptors.ascentOverride`](https://developer.mozilla.org/en-US/docs/Web/API/CSSFontFaceDescriptors/ascentOverride)
    pub fn set_ascent_override(&mut self, value: &CSSOMString) {
        self.inner.set("ascentOverride", value);
    }
}
impl CSSFontFaceDescriptors {
    /// Getter of the `descentOverride` attribute.
    /// [`CSSFontFaceDescriptors.descentOverride`](https://developer.mozilla.org/en-US/docs/Web/API/CSSFontFaceDescriptors/descentOverride)
    pub fn descent_override(&self) -> CSSOMString {
        self.inner.get("descentOverride").as_::<CSSOMString>()
    }

    /// Setter of the `descentOverride` attribute.
    /// [`CSSFontFaceDescriptors.descentOverride`](https://developer.mozilla.org/en-US/docs/Web/API/CSSFontFaceDescriptors/descentOverride)
    pub fn set_descent_override(&mut self, value: &CSSOMString) {
        self.inner.set("descentOverride", value);
    }
}
impl CSSFontFaceDescriptors {
    /// Getter of the `lineGapOverride` attribute.
    /// [`CSSFontFaceDescriptors.lineGapOverride`](https://developer.mozilla.org/en-US/docs/Web/API/CSSFontFaceDescriptors/lineGapOverride)
    pub fn line_gap_override(&self) -> CSSOMString {
        self.inner.get("lineGapOverride").as_::<CSSOMString>()
    }

    /// Setter of the `lineGapOverride` attribute.
    /// [`CSSFontFaceDescriptors.lineGapOverride`](https://developer.mozilla.org/en-US/docs/Web/API/CSSFontFaceDescriptors/lineGapOverride)
    pub fn set_line_gap_override(&mut self, value: &CSSOMString) {
        self.inner.set("lineGapOverride", value);
    }
}
impl CSSFontFaceDescriptors {
    /// Getter of the `superscriptPositionOverride` attribute.
    /// [`CSSFontFaceDescriptors.superscriptPositionOverride`](https://developer.mozilla.org/en-US/docs/Web/API/CSSFontFaceDescriptors/superscriptPositionOverride)
    pub fn superscript_position_override(&self) -> CSSOMString {
        self.inner
            .get("superscriptPositionOverride")
            .as_::<CSSOMString>()
    }

    /// Setter of the `superscriptPositionOverride` attribute.
    /// [`CSSFontFaceDescriptors.superscriptPositionOverride`](https://developer.mozilla.org/en-US/docs/Web/API/CSSFontFaceDescriptors/superscriptPositionOverride)
    pub fn set_superscript_position_override(&mut self, value: &CSSOMString) {
        self.inner.set("superscriptPositionOverride", value);
    }
}
impl CSSFontFaceDescriptors {
    /// Getter of the `subscriptPositionOverride` attribute.
    /// [`CSSFontFaceDescriptors.subscriptPositionOverride`](https://developer.mozilla.org/en-US/docs/Web/API/CSSFontFaceDescriptors/subscriptPositionOverride)
    pub fn subscript_position_override(&self) -> CSSOMString {
        self.inner
            .get("subscriptPositionOverride")
            .as_::<CSSOMString>()
    }

    /// Setter of the `subscriptPositionOverride` attribute.
    /// [`CSSFontFaceDescriptors.subscriptPositionOverride`](https://developer.mozilla.org/en-US/docs/Web/API/CSSFontFaceDescriptors/subscriptPositionOverride)
    pub fn set_subscript_position_override(&mut self, value: &CSSOMString) {
        self.inner.set("subscriptPositionOverride", value);
    }
}
impl CSSFontFaceDescriptors {
    /// Getter of the `superscriptSizeOverride` attribute.
    /// [`CSSFontFaceDescriptors.superscriptSizeOverride`](https://developer.mozilla.org/en-US/docs/Web/API/CSSFontFaceDescriptors/superscriptSizeOverride)
    pub fn superscript_size_override(&self) -> CSSOMString {
        self.inner
            .get("superscriptSizeOverride")
            .as_::<CSSOMString>()
    }

    /// Setter of the `superscriptSizeOverride` attribute.
    /// [`CSSFontFaceDescriptors.superscriptSizeOverride`](https://developer.mozilla.org/en-US/docs/Web/API/CSSFontFaceDescriptors/superscriptSizeOverride)
    pub fn set_superscript_size_override(&mut self, value: &CSSOMString) {
        self.inner.set("superscriptSizeOverride", value);
    }
}
impl CSSFontFaceDescriptors {
    /// Getter of the `subscriptSizeOverride` attribute.
    /// [`CSSFontFaceDescriptors.subscriptSizeOverride`](https://developer.mozilla.org/en-US/docs/Web/API/CSSFontFaceDescriptors/subscriptSizeOverride)
    pub fn subscript_size_override(&self) -> CSSOMString {
        self.inner.get("subscriptSizeOverride").as_::<CSSOMString>()
    }

    /// Setter of the `subscriptSizeOverride` attribute.
    /// [`CSSFontFaceDescriptors.subscriptSizeOverride`](https://developer.mozilla.org/en-US/docs/Web/API/CSSFontFaceDescriptors/subscriptSizeOverride)
    pub fn set_subscript_size_override(&mut self, value: &CSSOMString) {
        self.inner.set("subscriptSizeOverride", value);
    }
}

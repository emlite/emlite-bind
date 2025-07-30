use super::*;

/// The CSSFontPaletteValuesRule class.
/// [`CSSFontPaletteValuesRule`](https://developer.mozilla.org/en-US/docs/Web/API/CSSFontPaletteValuesRule)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct CSSFontPaletteValuesRule {
    inner: CSSRule,
}
impl FromVal for CSSFontPaletteValuesRule {
    fn from_val(v: &Any) -> Self {
        CSSFontPaletteValuesRule {
            inner: CSSRule::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for CSSFontPaletteValuesRule {
    type Target = CSSRule;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for CSSFontPaletteValuesRule {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for CSSFontPaletteValuesRule {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for CSSFontPaletteValuesRule {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<CSSFontPaletteValuesRule> for Any {
    fn from(s: CSSFontPaletteValuesRule) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&CSSFontPaletteValuesRule> for Any {
    fn from(s: &CSSFontPaletteValuesRule) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(CSSFontPaletteValuesRule);

impl CSSFontPaletteValuesRule {
    /// Getter of the `name` attribute.
    /// [`CSSFontPaletteValuesRule.name`](https://developer.mozilla.org/en-US/docs/Web/API/CSSFontPaletteValuesRule/name)
    pub fn name(&self) -> JsString {
        self.inner.get("name").as_::<JsString>()
    }
}
impl CSSFontPaletteValuesRule {
    /// Getter of the `fontFamily` attribute.
    /// [`CSSFontPaletteValuesRule.fontFamily`](https://developer.mozilla.org/en-US/docs/Web/API/CSSFontPaletteValuesRule/fontFamily)
    pub fn font_family(&self) -> JsString {
        self.inner.get("fontFamily").as_::<JsString>()
    }
}
impl CSSFontPaletteValuesRule {
    /// Getter of the `basePalette` attribute.
    /// [`CSSFontPaletteValuesRule.basePalette`](https://developer.mozilla.org/en-US/docs/Web/API/CSSFontPaletteValuesRule/basePalette)
    pub fn base_palette(&self) -> JsString {
        self.inner.get("basePalette").as_::<JsString>()
    }
}
impl CSSFontPaletteValuesRule {
    /// Getter of the `overrideColors` attribute.
    /// [`CSSFontPaletteValuesRule.overrideColors`](https://developer.mozilla.org/en-US/docs/Web/API/CSSFontPaletteValuesRule/overrideColors)
    pub fn override_colors(&self) -> JsString {
        self.inner.get("overrideColors").as_::<JsString>()
    }
}

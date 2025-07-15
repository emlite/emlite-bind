use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct CSSFontPaletteValuesRule {
    inner: CSSRule,
}
impl FromVal for CSSFontPaletteValuesRule {
    fn from_val(v: &emlite::Val) -> Self {
        CSSFontPaletteValuesRule {
            inner: CSSRule::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
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
impl AsRef<emlite::Val> for CSSFontPaletteValuesRule {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for CSSFontPaletteValuesRule {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<CSSFontPaletteValuesRule> for emlite::Val {
    fn from(s: CSSFontPaletteValuesRule) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&CSSFontPaletteValuesRule> for emlite::Val {
    fn from(s: &CSSFontPaletteValuesRule) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(CSSFontPaletteValuesRule);

impl CSSFontPaletteValuesRule {
    pub fn name(&self) -> CSSOMString {
        self.inner.get("name").as_::<CSSOMString>()
    }
}
impl CSSFontPaletteValuesRule {
    pub fn font_family(&self) -> CSSOMString {
        self.inner.get("fontFamily").as_::<CSSOMString>()
    }
}
impl CSSFontPaletteValuesRule {
    pub fn base_palette(&self) -> CSSOMString {
        self.inner.get("basePalette").as_::<CSSOMString>()
    }
}
impl CSSFontPaletteValuesRule {
    pub fn override_colors(&self) -> CSSOMString {
        self.inner.get("overrideColors").as_::<CSSOMString>()
    }
}

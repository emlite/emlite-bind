use super::*;

#[derive(Clone, Debug)]
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
impl std::ops::Deref for CSSFontPaletteValuesRule {
    type Target = CSSRule;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for CSSFontPaletteValuesRule {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<CSSFontPaletteValuesRule> for emlite::Val {
    fn from(s: CSSFontPaletteValuesRule) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl CSSFontPaletteValuesRule {
    pub fn name(&self) -> jsbind::CSSOMString {
        self.inner.get("name").as_::<jsbind::CSSOMString>()
    }
}
impl CSSFontPaletteValuesRule {
    pub fn font_family(&self) -> jsbind::CSSOMString {
        self.inner.get("fontFamily").as_::<jsbind::CSSOMString>()
    }
}
impl CSSFontPaletteValuesRule {
    pub fn base_palette(&self) -> jsbind::CSSOMString {
        self.inner.get("basePalette").as_::<jsbind::CSSOMString>()
    }
}
impl CSSFontPaletteValuesRule {
    pub fn override_colors(&self) -> jsbind::CSSOMString {
        self.inner
            .get("overrideColors")
            .as_::<jsbind::CSSOMString>()
    }
}
